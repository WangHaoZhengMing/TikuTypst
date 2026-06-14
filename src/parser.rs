use crate::model::{Metadata, QuestionFields, QuestionKind, Record};
use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;

pub fn parse_record(json: &str) -> Result<Record> {
    serde_json::from_str(json).context("invalid JSON")
}

pub fn parse_page_records(page: &str) -> Result<Vec<Record>> {
    let value = parse_page_value(page)?;
    let records = find_record_array(&value)
        .ok_or_else(|| {
            anyhow!(
                "page JSON must be an array or contain questions/questionDatas/list/records/items/rows"
            )
        })?
        .clone();
    serde_json::from_value(Value::Array(records)).context("invalid question record in page JSON")
}

fn parse_page_value(page: &str) -> Result<Value> {
    let trimmed = page.trim_start_matches('\u{feff}').trim_start();
    if trimmed.starts_with('[') || trimmed.starts_with('{') {
        return parse_relaxed_json(trimmed).context("invalid page JSON");
    }

    let assignment = trimmed
        .find("questionDatas")
        .ok_or_else(|| anyhow!("page is neither JSON nor a questionDatas assignment"))?;
    let array_start = trimmed[assignment..]
        .find('[')
        .map(|offset| assignment + offset)
        .ok_or_else(|| anyhow!("questionDatas assignment is missing an array"))?;
    let array = extract_array(&trimmed[array_start..])?;
    parse_relaxed_json(array).context("invalid questionDatas array")
}

fn parse_relaxed_json(json: &str) -> Result<Value> {
    match serde_json::from_str(json) {
        Ok(value) => Ok(value),
        Err(_) => serde_json::from_str(&remove_trailing_commas(json)).map_err(Into::into),
    }
}

fn extract_array(source: &str) -> Result<&str> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (index, character) in source.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        match character {
            '"' => in_string = true,
            '[' => depth += 1,
            ']' => {
                depth = depth
                    .checked_sub(1)
                    .ok_or_else(|| anyhow!("unexpected ] in questionDatas"))?;
                if depth == 0 {
                    return Ok(&source[..index + character.len_utf8()]);
                }
            }
            _ => {}
        }
    }
    bail!("questionDatas array is not closed")
}

fn remove_trailing_commas(json: &str) -> String {
    let characters = json.chars().collect::<Vec<_>>();
    let mut output = String::with_capacity(json.len());
    let mut in_string = false;
    let mut escaped = false;

    for (index, character) in characters.iter().copied().enumerate() {
        if in_string {
            output.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        if character == '"' {
            in_string = true;
            output.push(character);
            continue;
        }
        if character == ',' {
            let next = characters[index + 1..]
                .iter()
                .find(|next| !next.is_whitespace());
            if matches!(next, Some(']') | Some('}')) {
                continue;
            }
        }
        output.push(character);
    }
    output
}

fn find_record_array(value: &Value) -> Option<&Vec<Value>> {
    match value {
        Value::Array(items) => Some(items),
        Value::Object(object) => {
            for key in [
                "questions",
                "questionDatas",
                "list",
                "records",
                "items",
                "rows",
            ] {
                if let Some(items) = object.get(key).and_then(Value::as_array) {
                    return Some(items);
                }
            }
            for key in ["data", "page", "result"] {
                if let Some(records) = object.get(key).and_then(find_record_array) {
                    return Some(records);
                }
            }
            None
        }
        _ => None,
    }
}

pub(crate) fn extract_question(record: &Record) -> Result<QuestionFields> {
    extract_question_inner(record, false)
}

fn extract_question_inner(record: &Record, is_child: bool) -> Result<QuestionFields> {
    let kind = question_kind(record)?;
    if is_child && kind == QuestionKind::Composite {
        bail!("nested composite questions are not supported");
    }

    let children = record
        .small_questions
        .iter()
        .map(|child| extract_question_inner(child, true))
        .collect::<Result<Vec<_>>>()?;
    let metadata_children = children
        .iter()
        .map(|child| child.metadata.clone())
        .collect();

    Ok(QuestionFields {
        kind,
        stem: question_stem(record, kind),
        answer: record.question_info.answer.clone(),
        analysis: record.question_info.analysis.clone(),
        options: record.question_info.options.clone(),
        difficulty: record.difficulty,
        business_type: record.business_type.clone(),
        question_index: record.question_index.number(),
        metadata: Metadata {
            question_source: record.question_source.clone(),
            question_type: record.question_type.clone(),
            relation_type: record.relation_type.clone(),
            parent_id: optional_value(&record.parent_id),
            question_id: optional_value(&record.question_id),
            paper_id: record.paper_id.clone(),
            question_index: record.question_index.as_value(),
            sys_code: record.sys_code.clone(),
            standard_flag: record.standard_flag.clone(),
            input_type: record.input_type.clone(),
            add_flag: record.add_flag.clone(),
            flag_use: record.flag_use.clone(),
            knw_names: record
                .knw_infos
                .iter()
                .map(|info| info.knw_name.trim())
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .collect(),
            children: metadata_children,
        },
        children,
    })
}

pub(crate) fn question_kind(record: &Record) -> Result<QuestionKind> {
    let structure = record.structure_type.trim().to_ascii_lowercase();
    let business = record.business_type.to_ascii_uppercase();
    let kind = match structure.as_str() {
        "biaoti" | "title" => QuestionKind::Title,
        "danxuan" | "single_choice" => QuestionKind::SingleChoice,
        "duoxuan" | "multiple_choice" => QuestionKind::MultipleChoice,
        "fillblank" | "tiankong" | "fuhe_fillblank" => QuestionKind::FillBlank,
        "panduan" | "judge" => QuestionKind::Judge,
        "zhuguan" | "fuhe_zhuguan" | "subjective" => QuestionKind::Subjective,
        "fuhe" | "composite" => QuestionKind::Composite,
        "completion" | "wanxing" | "yueduhuanyuan" => QuestionKind::Completion,
        "" if !record.content.trim().is_empty() || !record.question_content.trim().is_empty() => {
            QuestionKind::Title
        }
        _ if business.contains("DANXUAN") => QuestionKind::SingleChoice,
        _ if business.contains("DUOXUAN") => QuestionKind::MultipleChoice,
        _ if business.contains("TIANKONG") => QuestionKind::FillBlank,
        _ if business.contains("PDT") || business.contains("PANDUAN") => QuestionKind::Judge,
        _ if business.contains("WANXING") || business.contains("HUANYUAN") => {
            QuestionKind::Completion
        }
        _ if !record.small_questions.is_empty() => QuestionKind::Composite,
        _ => {
            return Err(anyhow!(
                "unsupported question type structureType={:?}, businessType={:?}",
                record.structure_type,
                record.business_type
            ));
        }
    };
    Ok(kind)
}

fn question_stem(record: &Record, kind: QuestionKind) -> String {
    if !record.question_info.stem.trim().is_empty() {
        return record.question_info.stem.clone();
    }
    if kind == QuestionKind::Title && !record.question_content.trim().is_empty() {
        return record.question_content.clone();
    }
    record.content.clone()
}

fn optional_value(value: &Value) -> Option<Value> {
    match value {
        Value::Null => None,
        Value::String(value) if value.trim().is_empty() => None,
        value => Some(value.clone()),
    }
}
