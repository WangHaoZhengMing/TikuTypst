use anyhow::{Context, Result, anyhow, bail};
use regex::{Captures, Regex};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::LazyLock;

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
static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<[^>]+>").unwrap());
static LATEX_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)\$(.+?)\$").unwrap());
static CHOICE_BLANK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[（(]\s*(?:　|\u{2002}|\u{2003}|\u{2005}|\u{2009})*\s*[）)]").unwrap()
});
static LEADING_NUMBER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:(?:\(?\d+\)?[.、．])|(?:[（(]\d+[）)]))\s*").unwrap());
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
const TOKEN_SUFFIX: &str = "\u{e001}";
const LATEX_TOKEN_PREFIX: &str = "\u{e000}LATEX";

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub question_index: Index,
    #[serde(default)]
    pub business_type: String,
    #[serde(default)]
    pub business_type_name: String,
    #[serde(default)]
    pub structure_type: String,
    #[serde(default)]
    pub structure_type_name: String,
    #[serde(default)]
    pub difficulty: i64,
    #[serde(default)]
    pub question_info: QuestionInfo,
    #[serde(default)]
    pub small_questions: Vec<Record>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(untagged)]
pub enum Index {
    Number(i64),
    Text(String),
    #[default]
    Missing,
}

impl Index {
    pub fn number(&self) -> Option<i64> {
        match self {
            Self::Number(value) => Some(*value),
            Self::Text(value) => value.parse().ok(),
            Self::Missing => None,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct QuestionInfo {
    #[serde(default)]
    pub stem: String,
    #[serde(default)]
    pub analysis: String,
    #[serde(default)]
    pub answer: Value,
    #[serde(default)]
    pub options: Vec<OptionItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionItem {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub html_code: String,
    #[serde(default)]
    pub flag_answer: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Title,
    SingleChoice,
    MultipleChoice,
    FillBlank,
    Judge,
    Subjective,
    Composite,
    Completion,
}

#[derive(Debug, Clone, Copy)]
enum TextContext {
    Normal,
    ChoiceStem,
}

pub fn parse_record(json: &str) -> Result<Record> {
    serde_json::from_str(json).context("JSON 格式无效")
}

pub fn convert_json(json: &str) -> Result<String> {
    let record = parse_record(json)?;
    render_record(&record)
}

pub fn record_sort_key(record: &Record) -> (i64, i64) {
    if kind_of(record).ok() == Some(Kind::Title) {
        (0, 0)
    } else {
        (1, record.question_index.number().unwrap_or(i64::MAX))
    }
}

pub fn render_record(record: &Record) -> Result<String> {
    render_record_inner(record, false)
}

fn render_record_inner(record: &Record, is_child: bool) -> Result<String> {
    let kind = kind_of(record)?;
    if is_child && kind == Kind::Composite {
        bail!("复合题不能嵌套复合题");
    }

    match kind {
        Kind::Title => Ok(format!(
            "title({})",
            html_to_content(&record.content, TextContext::Normal)?
        )),
        Kind::SingleChoice | Kind::MultipleChoice => render_choice(record, kind),
        Kind::FillBlank => render_fill_blank(record),
        Kind::Judge => {
            let stem = html_to_content(&record.question_info.stem, TextContext::Normal)?;
            Ok(format!(
                "judge({stem}, difficulty: {}, business-type: {})",
                record.difficulty,
                typst_string(&business_name(record, kind))
            ))
        }
        Kind::Subjective => {
            let stem = html_to_content(&record.question_info.stem, TextContext::Normal)?;
            Ok(format!(
                "subj({stem}, difficulty: {}, business-type: {})",
                record.difficulty,
                typst_string(&business_name(record, kind))
            ))
        }
        Kind::Composite => render_composite(record),
        Kind::Completion => render_completion(record),
    }
}

fn render_choice(record: &Record, kind: Kind) -> Result<String> {
    let function = if kind == Kind::SingleChoice {
        "sc"
    } else {
        "mc"
    };
    let stem = html_to_content(&record.question_info.stem, TextContext::ChoiceStem)?;
    let options = render_options(&record.question_info.options)?;
    let answer = render_answer(&record.question_info.answer)?;
    let analysis = html_to_content(&record.question_info.analysis, TextContext::Normal)?;

    Ok(format!(
        "{function}({stem}, {options}, difficulty: {}, business-type: {}, answer: {answer}, analysis: {analysis})",
        record.difficulty,
        typst_string(&business_name(record, kind))
    ))
}

fn render_fill_blank(record: &Record) -> Result<String> {
    let stem = html_to_content(&record.question_info.stem, TextContext::Normal)?;
    if !stem.contains("#blank_line()") {
        bail!(
            "填空题 questionIndex={} 的 stem 缺少 #blank_line()",
            record.question_index.number().unwrap_or_default()
        );
    }
    let answer = render_answer(&record.question_info.answer)?;
    let analysis = html_to_content(&record.question_info.analysis, TextContext::Normal)?;
    Ok(format!(
        "fb({stem}, difficulty: {}, business-type: {}, answer: {answer}, analysis: {analysis})",
        record.difficulty,
        typst_string(&business_name(record, Kind::FillBlank))
    ))
}

fn render_composite(record: &Record) -> Result<String> {
    let stem = html_to_content(&record.question_info.stem, TextContext::Normal)?;
    let mut children = Vec::with_capacity(record.small_questions.len());
    for child in &record.small_questions {
        children.push(render_record_inner(child, true)?);
    }
    let tuple = render_call_tuple(&children);
    Ok(format!(
        "composite({stem}, {tuple}, difficulty: {}, business-type: {})",
        record.difficulty,
        typst_string(&business_name(record, Kind::Composite))
    ))
}

fn render_completion(record: &Record) -> Result<String> {
    let stem = html_to_content(&record.question_info.stem, TextContext::Normal)?;
    if !stem.contains("#blank_line()") {
        bail!(
            "阅读还原/完形题 questionIndex={} 的 stem 缺少 #blank_line()",
            record.question_index.number().unwrap_or_default()
        );
    }
    let options = render_options(&record.question_info.options)?;
    Ok(format!(
        "completion({stem}, {options}, difficulty: {}, business-type: {})",
        record.difficulty,
        typst_string(&business_name(record, Kind::Completion))
    ))
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
            bail!("选项标识 {label:?} 不是合法的 Typst 字段名");
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
        Value::String(value) => {
            let values: Vec<String> = value
                .split(|c: char| c == ',' || c == '，' || c.is_whitespace())
                .filter(|part| !part.is_empty())
                .map(|part| format!("[{}]", escape_typst_text(part)))
                .collect();
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

fn render_call_tuple(items: &[String]) -> String {
    if items.is_empty() {
        return "()".to_owned();
    }
    let body = items
        .iter()
        .map(|item| format!("  {item},"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("(\n{body}\n)")
}

fn kind_of(record: &Record) -> Result<Kind> {
    if !record.content.trim().is_empty() && record.structure_type.trim().is_empty() {
        return Ok(Kind::Title);
    }

    let structure = record.structure_type.to_ascii_lowercase();
    let business = record.business_type.to_ascii_uppercase();
    let kind = match structure.as_str() {
        "danxuan" | "single_choice" => Kind::SingleChoice,
        "duoxuan" | "multiple_choice" => Kind::MultipleChoice,
        "fillblank" | "tiankong" => Kind::FillBlank,
        "panduan" | "judge" => Kind::Judge,
        "zhuguan" | "fuhe_zhuguan" | "subjective" => Kind::Subjective,
        "fuhe" | "composite" => Kind::Composite,
        "completion" | "wanxing" | "yueduhuanyuan" => Kind::Completion,
        _ if business.contains("DANXUAN") => Kind::SingleChoice,
        _ if business.contains("DUOXUAN") => Kind::MultipleChoice,
        _ if business.contains("TIANKONG") => Kind::FillBlank,
        _ if business.contains("PDT") || business.contains("PANDUAN") => Kind::Judge,
        _ if business.contains("WANXING") || business.contains("HUANYUAN") => Kind::Completion,
        _ if !record.small_questions.is_empty() => Kind::Composite,
        _ => {
            return Err(anyhow!(
                "不支持的题型 structureType={:?}, businessType={:?}",
                record.structure_type,
                record.business_type
            ));
        }
    };
    Ok(kind)
}

fn business_name(record: &Record, kind: Kind) -> String {
    if !record.business_type_name.trim().is_empty() {
        return record.business_type_name.trim().to_owned();
    }
    match kind {
        Kind::Title => "",
        Kind::SingleChoice => "单项选择",
        Kind::MultipleChoice => "多项选择",
        Kind::FillBlank => "填空题",
        Kind::Judge => "判断题",
        Kind::Subjective => "解答题",
        Kind::Composite => "综合题",
        Kind::Completion => "阅读还原",
    }
    .to_owned()
}

fn html_to_content(html: &str, context: TextContext) -> Result<String> {
    if html.trim().is_empty() {
        return Ok("[]".to_owned());
    }

    let captures: Vec<String> = PARAGRAPH_RE
        .captures_iter(html)
        .map(|capture| capture[1].to_owned())
        .collect();
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
        let text = convert_fragment(&paragraph, context)?;
        if !text.trim().is_empty() {
            paragraphs.push(text.trim().to_owned());
        }
    }

    Ok(match paragraphs.as_slice() {
        [] => "[]".to_owned(),
        [only] => format!("[{only}]"),
        many => format!(
            "[{}]",
            many.iter()
                .map(|paragraph| format!("#par[{paragraph}]"))
                .collect::<String>()
        ),
    })
}

fn convert_fragment(fragment: &str, context: TextContext) -> Result<String> {
    let mut images = Vec::new();
    let mut latex = Vec::new();
    let mut text = BLANK_RE.replace_all(fragment, BLANK_TOKEN).into_owned();
    text = BREAK_RE.replace_all(&text, BREAK_TOKEN).into_owned();
    text = IMAGE_RE
        .replace_all(&text, |captures: &Captures| {
            let index = images.len();
            images.push(html_escape::decode_html_entities(&captures[1]).into_owned());
            format!("{IMAGE_TOKEN_PREFIX}{index}{TOKEN_SUFFIX}")
        })
        .into_owned();
    for captures in LATEX_RE.captures_iter(&text) {
        if captures[1].contains(BLANK_TOKEN) {
            bail!("#blank_line() 不能位于 #latex(`...`) 内部");
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
        bail!("发现未配对的 $ 数学公式分隔符");
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
    for (index, url) in images.iter().enumerate() {
        tokens.insert(
            format!("{IMAGE_TOKEN_PREFIX}{index}{TOKEN_SUFFIX}"),
            format!("#image({}, width: 100%)", typst_string(url)),
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
            .ok_or_else(|| anyhow!("内部占位符丢失: {:?}", matched.as_str()))?;
        output.push_str(replacement);
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
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            output.push_str(&latex_call(&format!(r"\rm{{{}}}", matched.as_str())));
        } else {
            output.push_str(&latex_call(matched.as_str()));
        }
        last = matched.end();
    }
    output.push_str(&escape_typst_text(&text[last..]));
    output
}

fn escape_typst_text(text: &str) -> String {
    text.replace('\\', "\\\\")
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

fn typst_string(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
    )
}

fn option_label(index: usize) -> String {
    let letter = char::from_u32('A' as u32 + index as u32).unwrap_or('A');
    letter.to_string()
}

fn is_typst_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_multiple_paragraphs_and_latex() {
        let content = html_to_content(
            r#"<p>第一段有$\\frac{1}{2}$。</p><p>重$3$kg。</p>"#,
            TextContext::Normal,
        )
        .unwrap();
        assert_eq!(
            content,
            r"[#par[第一段有#latex(`\frac{1}{2}`)。]#par[重#latex(`3`)#latex(`\rm{kg}`)。]]"
        );
    }

    #[test]
    fn converts_choice_parentheses() {
        let content = html_to_content("<p>答案是（　　）</p>", TextContext::ChoiceStem).unwrap();
        assert_eq!(content, "[答案是#klammern()]");
    }

    #[test]
    fn removes_question_number_before_converting_numbers() {
        let content = html_to_content("<p>12. 半径是3厘米。</p>", TextContext::Normal).unwrap();
        assert_eq!(content, "[半径是#latex(`3`)厘米。]");
    }

    #[test]
    fn converts_fill_blank_outside_latex() {
        let content = html_to_content(
            r#"<p>坐标为（<span class="fillblank"> </span>，<span class="fillblank"> </span>）</p>"#,
            TextContext::Normal,
        )
        .unwrap();
        assert_eq!(content, "[坐标为（#blank_line()，#blank_line()）]");
        assert!(!content.contains("#latex(`(#blank_line()"));
    }

    #[test]
    fn rejects_fill_blank_inside_latex() {
        let error = html_to_content(
            r#"<p>$x=<span class="fillblank"> </span>$</p>"#,
            TextContext::Normal,
        )
        .unwrap_err();
        assert!(error.to_string().contains("不能位于"));
    }

    #[test]
    fn rejects_unclosed_latex() {
        let error = html_to_content("<p>公式为$x+1。</p>", TextContext::Normal).unwrap_err();
        assert!(error.to_string().contains("未配对"));
    }

    #[test]
    fn rejects_fill_blank_without_blank_marker() {
        let record: Record = serde_json::from_str(
            r#"{
                "structureType": "fillblank",
                "difficulty": 2,
                "questionInfo": {"stem": "<p>没有空格</p>"}
            }"#,
        )
        .unwrap();
        assert!(
            render_record(&record)
                .unwrap_err()
                .to_string()
                .contains("缺少")
        );
    }

    #[test]
    fn rejects_nested_composite() {
        let record: Record = serde_json::from_str(
            r#"{
                "structureType": "fuhe",
                "questionInfo": {"stem": "<p>材料</p>"},
                "smallQuestions": [{
                    "structureType": "fuhe",
                    "questionInfo": {"stem": "<p>嵌套</p>"},
                    "smallQuestions": [{"structureType": "zhuguan"}]
                }]
            }"#,
        )
        .unwrap();
        assert!(
            render_record(&record)
                .unwrap_err()
                .to_string()
                .contains("不能嵌套")
        );
    }
}
