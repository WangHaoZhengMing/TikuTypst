use crate::document::{render_array, render_metadata_string_array};
use crate::model::{Metadata, OptionItem, QuestionFields, QuestionKind};
use anyhow::{Context, Result, anyhow, bail};
use regex::{Captures, Regex};
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

static TABLE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<table\b[^>]*>.*?</table\s*>").unwrap());
static PARAGRAPH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?is)<p\b[^>]*>(.*?)</p\s*>").unwrap());
static BREAK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<br\b[^>]*?/?>").unwrap());
static BLANK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?is)<span\b[^>]*class\s*=\s*["'][^"']*(?:fillblank|underline)[^"']*["'][^>]*>.*?</span\s*>"#,
    )
    .unwrap()
});
static IMAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?is)<img\b[^>]*\bsrc\s*=\s*["']([^"']+)["'][^>]*>"#).unwrap());
static IMAGE_WIDTH_ATTR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\bwidth\s*=\s*["']?\s*([0-9]+(?:\.[0-9]+)?)\s*(%|px)?["']?"#).unwrap()
});
static IMAGE_STYLE_WIDTH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)\bwidth\s*:\s*([0-9]+(?:\.[0-9]+)?)\s*(%|px)?"#).unwrap());
static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<[^>]+>").unwrap());
static LATEX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)\$(.+?)\$").unwrap());
static CHOICE_BLANK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\(\u{ff08}]\s*[\u{3000}\u{2002}\u{2003}\u{2005}\u{2009}]*\s*[\)\u{ff09}]")
        .unwrap()
});
static LEADING_NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(?:(?:\(?\d+\)?[.\u{3001}\u{ff0e}])|(?:[\(\u{ff08}]\d+[\)\u{ff09}]))\s*")
        .unwrap()
});
static PLAIN_ATOM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[A-Za-z]+(?:[A-Za-z0-9_-]*[A-Za-z0-9])?\b|\d+(?:\.\d+)?%?").unwrap()
});
static WHITESPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\t\r\n ]+").unwrap());
static COMMAND_SLASH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\+([A-Za-z]+|[%#&_])").unwrap());
static INTERNAL_TOKEN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\u{e000}(?:BLANK|BREAK|KLAMMERN|IMAGE\d+|LATEX\d+)\u{e001}").unwrap()
});

const BLANK_TOKEN: &str = "\u{e000}BLANK\u{e001}";
const BREAK_TOKEN: &str = "\u{e000}BREAK\u{e001}";
const KLAMMERN_TOKEN: &str = "\u{e000}KLAMMERN\u{e001}";
const IMAGE_TOKEN_PREFIX: &str = "\u{e000}IMAGE";
const LATEX_TOKEN_PREFIX: &str = "\u{e000}LATEX";
const TOKEN_SUFFIX: &str = "\u{e001}";
const IMAGE_CONTAINER_WIDTH_PX: f64 = 698.6;

#[derive(Debug, Clone, Copy)]
enum TextContext {
    Normal,
    ChoiceStem,
}

pub(crate) fn render_question(question: &QuestionFields) -> Result<String> {
    render_question_data(question)
}

fn render_question_data(question: &QuestionFields) -> Result<String> {
    let kind = question_type_name(question.kind);
    let mut fields = vec![format!("type: {}", crate::typst_string(kind))];

    match question.kind {
        QuestionKind::Title => {
            fields.push(format!(
                "content: {}",
                html_to_content(&question.stem, TextContext::Normal)?
            ));
        }
        QuestionKind::SingleChoice | QuestionKind::MultipleChoice => {
            fields.push(format!(
                "content: {}",
                html_to_content(&question.stem, TextContext::ChoiceStem)?
            ));
            fields.push(format!("options: {}", render_options(&question.options)?));
            push_common_question_fields(&mut fields, question)?;
        }
        QuestionKind::FillBlank => {
            let stem = html_to_content(&question.stem, TextContext::Normal)?;
            if !stem.contains("#blank_line()") {
                bail!(
                    "fill-blank questionIndex={} is missing #blank_line() in stem",
                    question.question_index.unwrap_or_default()
                );
            }
            fields.push(format!("content: {stem}"));
            push_common_question_fields(&mut fields, question)?;
        }
        QuestionKind::Judge => {
            fields.push(format!(
                "content: {}",
                html_to_content(&question.stem, TextContext::Normal)?
            ));
            push_common_question_fields(&mut fields, question)?;
        }
        QuestionKind::Subjective => {
            fields.push(format!(
                "content: {}",
                html_to_content(&question.stem, TextContext::Normal)?
            ));
            push_common_question_fields(&mut fields, question)?;
        }
        QuestionKind::Composite => {
            fields.push(format!(
                "content: {}",
                html_to_content(&question.stem, TextContext::Normal)?
            ));
            let children = question
                .children
                .iter()
                .map(render_question_data)
                .collect::<Result<Vec<_>>>()?;
            fields.push(format!(
                "children: {}",
                render_array(children.iter().map(String::as_str))
            ));
            fields.push(format!("difficulty: {}", question.difficulty));
            fields.push(format!(
                "business-type: {}",
                crate::typst_string(&question.business_type)
            ));
        }
        QuestionKind::Completion => {
            let stem = html_to_content(&question.stem, TextContext::Normal)?;
            if !stem.contains("#blank_line()") {
                bail!(
                    "completion questionIndex={} is missing #blank_line() in stem",
                    question.question_index.unwrap_or_default()
                );
            }
            fields.push(format!("content: {stem}"));
            fields.push(format!("options: {}", render_options(&question.options)?));
            fields.push(format!("difficulty: {}", question.difficulty));
            fields.push(format!(
                "business-type: {}",
                crate::typst_string(&question.business_type)
            ));
        }
    }

    fields.push(format!("meta: {}", render_metadata(&question.metadata)));
    Ok(format!(
        "(\n{}\n)",
        fields
            .iter()
            .map(|field| format!("  {field},"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
}

fn push_common_question_fields(fields: &mut Vec<String>, question: &QuestionFields) -> Result<()> {
    fields.push(format!("difficulty: {}", question.difficulty));
    fields.push(format!(
        "business-type: {}",
        crate::typst_string(&question.business_type)
    ));
    fields.push(format!("answer: {}", render_answer(&question.answer)?));
    fields.push(format!(
        "analysis: {}",
        html_to_content(&question.analysis, TextContext::Normal)?
    ));
    Ok(())
}

fn question_type_name(kind: QuestionKind) -> &'static str {
    match kind {
        QuestionKind::Title => "title",
        QuestionKind::SingleChoice => "sc",
        QuestionKind::MultipleChoice => "mc",
        QuestionKind::FillBlank => "fb",
        QuestionKind::Judge => "judge",
        QuestionKind::Subjective => "subj",
        QuestionKind::Composite => "composite",
        QuestionKind::Completion => "completion",
    }
}

fn render_metadata(metadata: &Metadata) -> String {
    let mut fields = vec![
        metadata_field("questionSource", &metadata.question_source),
        metadata_field("questionType", &metadata.question_type),
        metadata_field("relationType", &metadata.relation_type),
    ];
    if let Some(parent_id) = &metadata.parent_id {
        fields.push(metadata_field("parentId", parent_id));
    }
    if let Some(question_id) = &metadata.question_id {
        fields.push(metadata_field("questionId", question_id));
    }
    fields.extend([
        metadata_field("paperId", &metadata.paper_id),
        metadata_field("questionIndex", &metadata.question_index),
        metadata_field("sysCode", &metadata.sys_code),
        metadata_field("standardFlag", &metadata.standard_flag),
        metadata_field("inputType", &metadata.input_type),
        metadata_field("addFlag", &metadata.add_flag),
        metadata_field("flagUse", &metadata.flag_use),
        format!(
            "knwNames: {}",
            render_metadata_string_array(&metadata.knw_names)
        ),
    ]);
    if !metadata.children.is_empty() {
        let children = metadata
            .children
            .iter()
            .map(render_metadata)
            .collect::<Vec<_>>()
            .join(", ");
        fields.push(format!("smallQuestions: ({children},)"));
    }

    format!(
        "({})",
        fields
            .iter()
            .map(|field| format!("{field},"))
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn metadata_field(name: &str, value: &Value) -> String {
    format!("{name}: {}", render_metadata_value(value))
}

fn render_metadata_value(value: &Value) -> String {
    match value {
        Value::Null => "none".to_owned(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => crate::typst_string(value),
        other => crate::typst_string(&other.to_string()),
    }
}

fn render_options(options: &[OptionItem]) -> Result<String> {
    let mut rendered = Vec::with_capacity(options.len());
    for (index, option) in options.iter().enumerate() {
        let label = if option.title.trim().is_empty() {
            option_label(index)
        } else {
            option.title.trim().to_owned()
        };
        if !is_typst_identifier(&label) {
            bail!("option label {label:?} is not a valid Typst field name");
        }
        let content = html_to_content(&option.html_code, TextContext::Normal)?;
        rendered.push(format!("{label}: {content}"));
    }
    Ok(format!("({})", rendered.join(", ")))
}

fn render_answer(answer: &Value) -> Result<String> {
    match answer {
        Value::Null => Ok("()".to_owned()),
        Value::String(value) if value.trim().is_empty() => Ok("()".to_owned()),
        Value::String(value) if value.contains('<') || value.contains('$') => Ok(
            render_content_tuple(&[html_to_content(value, TextContext::Normal)?]),
        ),
        Value::String(value) => {
            let values = value
                .split(|character: char| {
                    character == ',' || character == '\u{ff0c}' || character.is_whitespace()
                })
                .filter(|part| !part.is_empty())
                .map(|part| format!("[{}]", escape_typst_text(part)))
                .collect::<Vec<_>>();
            Ok(render_content_tuple(&values))
        }
        Value::Array(items) => {
            let mut values = Vec::new();
            for item in items {
                if let Some(answers) = item.get("answers").and_then(Value::as_array) {
                    let first = answers.first().and_then(Value::as_str).unwrap_or_default();
                    values.push(html_to_content(first, TextContext::Normal)?);
                } else if let Some(value) = item.as_str() {
                    values.push(html_to_content(value, TextContext::Normal)?);
                }
            }
            Ok(render_content_tuple(&values))
        }
        other => Ok(format!("([{}],)", escape_typst_text(&other.to_string()))),
    }
}

fn render_content_tuple(items: &[String]) -> String {
    if items.is_empty() {
        "()".to_owned()
    } else {
        format!("({},)", items.join(", "))
    }
}

fn html_to_content(html: &str, context: TextContext) -> Result<String> {
    html_to_content_with_table_converter(html, context, &pandoc_table_to_typst)
}

fn html_to_content_with_table_converter(
    html: &str,
    context: TextContext,
    convert_table: &dyn Fn(&str) -> Result<String>,
) -> Result<String> {
    if html.trim().is_empty() {
        return Ok("[]".to_owned());
    }
    if TABLE_RE.is_match(html) {
        return html_with_tables_to_content(html, context, convert_table);
    }

    let captures = PARAGRAPH_RE
        .captures_iter(html)
        .map(|capture| capture[1].to_owned())
        .collect::<Vec<_>>();
    let raw_paragraphs = if captures.is_empty() {
        vec![html.to_owned()]
    } else {
        captures
    };

    let mut paragraphs = Vec::new();
    for (index, paragraph) in raw_paragraphs.iter().enumerate() {
        let paragraph = if index == 0 {
            LEADING_NUMBER_RE.replace(paragraph, "")
        } else {
            paragraph.as_str().into()
        };
        let content = convert_paragraph_content(&paragraph, context)?;
        if !content.trim().is_empty() {
            paragraphs.push(content);
        }
    }

    Ok(match paragraphs.as_slice() {
        [] => "[]".to_owned(),
        [only] => format!("[{only}]"),
        many => format!("[{}]", many.join("\n\n")),
    })
}

fn convert_paragraph_content(fragment: &str, context: TextContext) -> Result<String> {
    let mut content = String::new();
    let mut last = 0;
    for image in IMAGE_RE.captures_iter(fragment) {
        let whole = image
            .get(0)
            .ok_or_else(|| anyhow!("image capture is missing the full match"))?;
        content.push_str(&convert_fragment(&fragment[last..whole.start()], context)?);
        let url = html_escape::decode_html_entities(&image[1]);
        content.push_str(&render_image(&url, image_width(whole.as_str())));
        last = whole.end();
    }
    content.push_str(&convert_fragment(&fragment[last..], context)?);
    Ok(content.trim().to_owned())
}

fn image_width(image_tag: &str) -> Option<String> {
    let captures = IMAGE_STYLE_WIDTH_RE
        .captures(image_tag)
        .or_else(|| IMAGE_WIDTH_ATTR_RE.captures(image_tag))?;
    let value = captures.get(1)?.as_str().parse::<f64>().ok()?;
    let unit = captures
        .get(2)
        .map(|unit| unit.as_str().to_ascii_lowercase());
    let percentage = if unit.as_deref() == Some("%") {
        value
    } else {
        value / IMAGE_CONTAINER_WIDTH_PX * 100.0
    };
    Some(format_percentage(percentage))
}

fn format_percentage(value: f64) -> String {
    let formatted = format!("{value:.2}");
    format!("{}%", formatted.trim_end_matches('0').trim_end_matches('.'))
}

fn render_image(url: &str, width: Option<String>) -> String {
    format!(
        "#box(baseline: 25%, image({}, width: {}))",
        crate::typst_string(url),
        width.as_deref().unwrap_or("100%")
    )
}

fn html_with_tables_to_content(
    html: &str,
    context: TextContext,
    convert_table: &dyn Fn(&str) -> Result<String>,
) -> Result<String> {
    let mut body = String::new();
    let mut last = 0;
    for table in TABLE_RE.find_iter(html) {
        append_content_body(
            &mut body,
            &html_to_content_with_table_converter(
                &html[last..table.start()],
                context,
                convert_table,
            )?,
        );
        let converted = convert_table_preserving_blanks(table.as_str(), convert_table)?;
        if !body.is_empty() && !body.ends_with('\n') {
            body.push('\n');
        }
        body.push_str(converted.trim());
        body.push('\n');
        last = table.end();
    }
    append_content_body(
        &mut body,
        &html_to_content_with_table_converter(&html[last..], context, convert_table)?,
    );
    Ok(format!("[{}]", body.trim()))
}

fn convert_table_preserving_blanks(
    table_html: &str,
    convert_table: &dyn Fn(&str) -> Result<String>,
) -> Result<String> {
    let blank_count = BLANK_RE.find_iter(table_html).count();
    let prepared = BLANK_RE.replace_all(table_html, BLANK_TOKEN);
    let converted = convert_table(&prepared)?;
    let converted_blank_count = converted.matches(BLANK_TOKEN).count();
    if converted_blank_count != blank_count {
        bail!(
            "table conversion lost fill-blank placeholders: expected {blank_count}, got {converted_blank_count}"
        );
    }
    Ok(converted.replace(BLANK_TOKEN, "#blank_line()"))
}

fn append_content_body(output: &mut String, content: &str) {
    let body = content
        .strip_prefix('[')
        .and_then(|content| content.strip_suffix(']'))
        .unwrap_or(content);
    if body.trim().is_empty() {
        return;
    }
    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }
    output.push_str(body);
    output.push('\n');
}

fn pandoc_table_to_typst(table_html: &str) -> Result<String> {
    let executable = std::env::var_os("TIKU_TYPST_PANDOC").unwrap_or_else(|| "pandoc".into());
    let mut child = Command::new(&executable)
        .args(["--from=html", "--to=typst", "--wrap=none"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| {
            format!(
                "failed to start Pandoc {:?}; install Pandoc or set TIKU_TYPST_PANDOC",
                executable
            )
        })?;

    child
        .stdin
        .take()
        .ok_or_else(|| anyhow!("failed to open Pandoc stdin"))?
        .write_all(table_html.as_bytes())
        .context("failed to send HTML table to Pandoc")?;

    let output = child
        .wait_with_output()
        .context("failed while waiting for Pandoc")?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "Pandoc failed to convert HTML table (status {}): {}",
            output.status,
            stderr.trim()
        );
    }

    let typst = String::from_utf8(output.stdout).context("Pandoc returned non-UTF-8 output")?;
    if typst.trim().is_empty() {
        bail!("Pandoc returned empty Typst for an HTML table");
    }
    Ok(typst)
}

fn convert_fragment(fragment: &str, context: TextContext) -> Result<String> {
    let mut images = Vec::new();
    let mut latex = Vec::new();
    let mut text = BLANK_RE.replace_all(fragment, BLANK_TOKEN).into_owned();
    text = BREAK_RE.replace_all(&text, BREAK_TOKEN).into_owned();
    text = IMAGE_RE
        .replace_all(&text, |captures: &Captures| {
            let index = images.len();
            let whole = captures
                .get(0)
                .map(|value| value.as_str())
                .unwrap_or_default();
            images.push((
                html_escape::decode_html_entities(&captures[1]).into_owned(),
                image_width(whole),
            ));
            format!("{IMAGE_TOKEN_PREFIX}{index}{TOKEN_SUFFIX}")
        })
        .into_owned();
    for captures in LATEX_RE.captures_iter(&text) {
        if captures[1].contains(BLANK_TOKEN) {
            bail!("#blank_line() cannot be placed inside #latex(`...`)");
        }
    }
    text = LATEX_RE
        .replace_all(&text, |captures: &Captures| {
            let index = latex.len();
            let formula = html_escape::decode_html_entities(&captures[1]);
            latex.push(normalize_latex(&formula));
            format!("{LATEX_TOKEN_PREFIX}{index}{TOKEN_SUFFIX}")
        })
        .into_owned();
    if text.contains('$') {
        bail!("unmatched $ formula delimiter");
    }
    text = TAG_RE.replace_all(&text, "").into_owned();
    text = html_escape::decode_html_entities(&text).into_owned();
    text = text.replace('\u{00a0}', " ");
    text = WHITESPACE_RE.replace_all(&text, " ").into_owned();

    if matches!(context, TextContext::ChoiceStem) {
        text = CHOICE_BLANK_RE
            .replace_all(&text, KLAMMERN_TOKEN)
            .into_owned();
    }

    let mut tokens = BTreeMap::new();
    tokens.insert(BLANK_TOKEN.to_owned(), "#blank_line()".to_owned());
    tokens.insert(BREAK_TOKEN.to_owned(), "#linebreak()".to_owned());
    tokens.insert(KLAMMERN_TOKEN.to_owned(), "#klammern()".to_owned());
    for (index, (url, width)) in images.iter().enumerate() {
        tokens.insert(
            format!("{IMAGE_TOKEN_PREFIX}{index}{TOKEN_SUFFIX}"),
            render_image(url, width.clone()),
        );
    }
    for (index, formula) in latex.iter().enumerate() {
        tokens.insert(
            format!("{LATEX_TOKEN_PREFIX}{index}{TOKEN_SUFFIX}"),
            latex_call(formula),
        );
    }

    convert_plain_segments(&text, &tokens)
}

fn convert_plain_segments(text: &str, tokens: &BTreeMap<String, String>) -> Result<String> {
    let mut output = String::new();
    let mut last = 0;
    for matched in INTERNAL_TOKEN_RE.find_iter(text) {
        output.push_str(&convert_plain_text(&text[last..matched.start()]));
        let replacement = tokens
            .get(matched.as_str())
            .ok_or_else(|| anyhow!("missing internal token {:?}", matched.as_str()))?;
        output.push_str(replacement);
        if matched.as_str().starts_with(LATEX_TOKEN_PREFIX)
            && text[matched.end()..].starts_with(['.', '('])
        {
            output.push('\u{200b}');
        }
        last = matched.end();
    }
    output.push_str(&convert_plain_text(&text[last..]));
    Ok(output)
}

fn convert_plain_text(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut last = 0;
    for matched in PLAIN_ATOM_RE.find_iter(text) {
        output.push_str(&escape_typst_text(&text[last..matched.start()]));
        if matched
            .as_str()
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic())
        {
            output.push_str(&latex_call(&format!(r"\rm{{{}}}", matched.as_str())));
        } else {
            output.push_str(&latex_call(matched.as_str()));
        }
        if text[matched.end()..].starts_with(['.', '(']) {
            output.push('\u{200b}');
        }
        last = matched.end();
    }
    output.push_str(&escape_typst_text(&text[last..]));
    output
}

fn escape_typst_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('/', "\\/")
        .replace('#', "\\#")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

fn normalize_latex(formula: &str) -> String {
    COMMAND_SLASH_RE
        .replace_all(formula.trim(), r"\$1")
        .into_owned()
}

fn latex_call(formula: &str) -> String {
    format!("#latex(`{formula}`)")
}

fn option_label(index: usize) -> String {
    char::from_u32('A' as u32 + index as u32)
        .unwrap_or('A')
        .to_string()
}

fn is_typst_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn converts_choice_parentheses() {
        let content = html_to_content(
            "<p>answer is (\u{3000}\u{3000})</p>",
            TextContext::ChoiceStem,
        )
        .unwrap();
        assert_eq!(
            content,
            "[#latex(`\\rm{answer}`) #latex(`\\rm{is}`) #klammern()]"
        );
    }

    #[test]
    fn converts_fill_blank_outside_latex() {
        let content = html_to_content(
            r#"<p>x=<span class="fillblank"> </span></p>"#,
            TextContext::Normal,
        )
        .unwrap();
        assert_eq!(content, "[#latex(`\\rm{x}`)=#blank_line()]");
    }

    #[test]
    fn rejects_fill_blank_inside_latex() {
        let error = html_to_content(
            r#"<p>$x=<span class="fillblank"> </span>$</p>"#,
            TextContext::Normal,
        )
        .unwrap_err();
        assert!(error.to_string().contains("cannot be placed"));
    }

    #[test]
    fn converts_multiple_paragraphs_and_latex() {
        let content = html_to_content(
            r#"<p>first $\frac{1}{2}$.</p><p>weight $3$ kg.</p>"#,
            TextContext::Normal,
        )
        .unwrap();
        assert_eq!(
            content,
            "[#latex(`\\rm{first}`) #latex(`\\frac{1}{2}`)\u{200b}.\n\n#latex(`\\rm{weight}`) #latex(`3`) #latex(`\\rm{kg}`)\u{200b}.]"
        );
    }

    #[test]
    fn rejects_unclosed_latex() {
        let error = html_to_content("<p>formula $x+1.</p>", TextContext::Normal).unwrap_err();
        assert!(error.to_string().contains("unmatched"));
    }

    #[test]
    fn escapes_typst_comment_markers_in_plain_text() {
        let content = html_to_content("<p>AB//CD and A/*B</p>", TextContext::Normal).unwrap();
        assert_eq!(
            content,
            "[#latex(`\\rm{AB}`)\\/\\/#latex(`\\rm{CD}`) #latex(`\\rm{and}`) #latex(`\\rm{A}`)\\/*#latex(`\\rm{B}`)]"
        );
        assert!(!content.contains("//"));
    }

    #[test]
    fn delegates_only_html_tables_to_pandoc() {
        let converted_tables = RefCell::new(Vec::new());
        let converter = |table: &str| {
            converted_tables.borrow_mut().push(table.to_owned());
            Ok("#table(columns: 2, [A], [B])".to_owned())
        };
        let content = html_to_content_with_table_converter(
            "<p>before $2$</p><table><tr><td>A</td><td>B</td></tr></table><p>after (\u{3000}\u{3000})</p>",
            TextContext::ChoiceStem,
            &converter,
        )
        .unwrap();

        assert_eq!(converted_tables.borrow().len(), 1);
        assert_eq!(
            converted_tables.borrow()[0],
            "<table><tr><td>A</td><td>B</td></tr></table>"
        );
        assert!(content.contains("#latex(`\\rm{before}`) #latex(`2`)"));
        assert!(content.contains("#table(columns: 2, [A], [B])"));
        assert!(content.contains("#latex(`\\rm{after}`) #klammern()"));
    }

    #[test]
    fn supports_multiple_tables() {
        let converter = |_: &str| Ok("#table(columns: 1, [cell])".to_owned());
        let content = html_to_content_with_table_converter(
            "<table><tr><td>1</td></tr></table><p>middle</p><table><tr><td>2</td></tr></table>",
            TextContext::Normal,
            &converter,
        )
        .unwrap();

        assert_eq!(content.matches("#table(").count(), 2);
        assert!(content.contains("#latex(`\\rm{middle}`)"));
    }

    #[test]
    fn preserves_fill_blanks_inside_tables() {
        let converted_tables = RefCell::new(Vec::new());
        let converter = |table: &str| {
            converted_tables.borrow_mut().push(table.to_owned());
            Ok(format!(
                "#table(columns: 2, [first {BLANK_TOKEN}], [second {BLANK_TOKEN}])"
            ))
        };
        let content = html_to_content_with_table_converter(
            r#"<table><tr><td>first <span class="underline fillblank"> </span></td><td>second <span class="fillblank"> </span></td></tr></table>"#,
            TextContext::Normal,
            &converter,
        )
        .unwrap();

        let converted_table = &converted_tables.borrow()[0];
        assert_eq!(converted_table.matches(BLANK_TOKEN).count(), 2);
        assert!(!converted_table.contains("fillblank"));
        assert_eq!(content.matches("#blank_line()").count(), 2);
        assert!(!content.contains(BLANK_TOKEN));
    }

    #[test]
    fn rejects_table_conversion_that_loses_fill_blanks() {
        let converter = |_: &str| Ok("#table(columns: 1, [cell])".to_owned());
        let error = html_to_content_with_table_converter(
            r#"<table><tr><td><span class="fillblank"> </span></td></tr></table>"#,
            TextContext::Normal,
            &converter,
        )
        .unwrap_err();

        assert!(error.to_string().contains("expected 1, got 0"));
    }

    #[test]
    fn keeps_images_outside_paragraphs() {
        let content = html_to_content(
            r#"<p>before</p><p><img src="https://example.com/a.png"></p><p>after</p>"#,
            TextContext::Normal,
        )
        .unwrap();

        assert_eq!(
            content,
            "[#latex(`\\rm{before}`)\n\n#box(baseline: 25%, image(\"https://example.com/a.png\", width: 100%))\n\n#latex(`\\rm{after}`)]"
        );
        assert!(!content.contains("#par["));
    }

    #[test]
    fn splits_inline_images_from_surrounding_text() {
        let content = html_to_content(
            r#"<p>left<img src="https://example.com/a.png">right</p>"#,
            TextContext::Normal,
        )
        .unwrap();

        assert_eq!(
            content,
            "[#latex(`\\rm{left}`)#box(baseline: 25%, image(\"https://example.com/a.png\", width: 100%))#latex(`\\rm{right}`)]"
        );
        assert!(!content.contains("#par["));
    }

    #[test]
    fn renders_an_image_only_without_paragraph() {
        let content = html_to_content(
            r#"<p><img src="https://example.com/a.png"></p>"#,
            TextContext::Normal,
        )
        .unwrap();

        assert_eq!(
            content,
            "[#box(baseline: 25%, image(\"https://example.com/a.png\", width: 100%))]"
        );
    }

    #[test]
    fn converts_image_pixel_width_to_container_percentage() {
        let content = html_to_content(
            r#"<p><img height="127" src="https://example.com/a.png" width="118"></p>"#,
            TextContext::Normal,
        )
        .unwrap();

        assert_eq!(
            content,
            "[#box(baseline: 25%, image(\"https://example.com/a.png\", width: 16.89%))]"
        );
    }

    #[test]
    fn converts_image_style_width_to_container_percentage() {
        let content = html_to_content(
            r#"<p><img src="https://example.com/a.png" style="height: 20px; width: 349.3px"></p>"#,
            TextContext::Normal,
        )
        .unwrap();

        assert_eq!(
            content,
            "[#box(baseline: 25%, image(\"https://example.com/a.png\", width: 50%))]"
        );
    }

    #[test]
    fn preserves_image_percentage_width() {
        let content = html_to_content(
            r#"<p><img src="https://example.com/a.png" width="42.5%"></p>"#,
            TextContext::Normal,
        )
        .unwrap();

        assert_eq!(
            content,
            "[#box(baseline: 25%, image(\"https://example.com/a.png\", width: 42.5%))]"
        );
    }

    #[test]
    fn inserts_zero_width_space_after_latex_before_dot_or_parenthesis() {
        let dot = html_to_content("<p>$x$.</p>", TextContext::Normal).unwrap();
        let parenthesis = html_to_content("<p>$x$(next)</p>", TextContext::Normal).unwrap();

        assert_eq!(dot, "[#latex(`x`)\u{200b}.]");
        assert_eq!(parenthesis, "[#latex(`x`)\u{200b}(#latex(`\\rm{next}`))]");
    }

    #[test]
    fn does_not_insert_zero_width_space_before_other_characters() {
        let content = html_to_content("<p>$x$, next</p>", TextContext::Normal).unwrap();

        assert_eq!(content, "[#latex(`x`), #latex(`\\rm{next}`)]");
    }
}
