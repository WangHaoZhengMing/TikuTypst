mod document;
mod model;
mod parser;
mod render;

use anyhow::Result;

pub use model::Record;
pub use parser::{parse_page_records, parse_record};

pub fn convert_json(json: &str) -> Result<String> {
    let record = parse_record(json)?;
    let converted = convert_record(&record)?;
    Ok(document::render_document(&[converted]))
}

pub fn convert_page_json(json: &str) -> Result<String> {
    let records = parse_page_records(json)?;
    render_records(&records)
}

pub fn convert_record(record: &Record) -> Result<String> {
    let question = parser::extract_question(record)?;
    render::render_question(&question)
}

pub fn render_record(record: &Record) -> Result<String> {
    convert_record(record)
}

pub fn render_records(records: &[Record]) -> Result<String> {
    let converted = records
        .iter()
        .map(convert_record)
        .collect::<Result<Vec<_>>>()?;
    Ok(document::render_document(&converted))
}

pub fn record_sort_key(record: &Record) -> i64 {
    record.question_index.number().unwrap_or(i64::MAX)
}

pub(crate) fn typst_string(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_render_and_metadata_from_parsed_fields() {
        let json = r#"{
            "structureType": "danxuan",
            "businessType": "CSX-DANXUAN",
            "difficulty": 3,
            "questionInfo": {
                "stem": "<p>Choose (\u3000\u3000)</p>",
                "answer": "B",
                "analysis": "<p>analysis</p>",
                "options": [
                    {"title": "A", "htmlCode": "<p>one</p>"},
                    {"title": "B", "htmlCode": "<p>two</p>"}
                ]
            },
            "questionSource": 2,
            "questionType": 1,
            "relationType": 1,
            "parentId": "2833099944152530944",
            "questionId": "2833099944152530944",
            "paperId": "3635497690720632832",
            "questionIndex": 2,
            "sysCode": 1,
            "standardFlag": 1,
            "inputType": 1,
            "addFlag": 1,
            "flagUse": 1,
            "knwInfos": [
                {"knwId": "1", "knwName": "Knowledge A"},
                {"knwId": "2", "knwName": "Knowledge B"}
            ]
        }"#;

        let output = convert_json(json).unwrap();
        assert!(output.contains("type: \"sc\""));
        assert!(output.contains("content: [#latex(`\\rm{Choose}`) #klammern()]"));
        assert!(output.contains("options: (A: [#latex(`\\rm{one}`)], B: [#latex(`\\rm{two}`)])"));
        assert!(output.contains("business-type: \"CSX-DANXUAN\""));
        assert!(output.contains("meta: ("));
        assert!(!output.contains("meta: (\n"));
        assert!(output.contains("questionSource: 2"));
        assert!(output.contains("parentId: \"2833099944152530944\""));
        assert!(output.contains("paperId: \"3635497690720632832\""));
        assert!(output.contains("knwNames: (\"Knowledge A\", \"Knowledge B\",)"));
        assert!(!output.contains("knwId"));
        assert!(output.ends_with("#paper_data"));
    }

    #[test]
    fn omits_empty_optional_ids_but_keeps_required_metadata_keys() {
        let json = r#"{
            "structureType": "judge",
            "businessType": "CSX-PDT",
            "questionInfo": {"stem": "<p>True.</p>"},
            "parentId": "",
            "questionId": null
        }"#;

        let output = convert_json(json).unwrap();
        assert!(!output.contains("parentId:"));
        assert!(!output.contains("questionId:"));
        assert!(output.contains("questionSource: none"));
        assert!(output.contains("standardFlag: none"));
        assert!(output.contains("flagUse: none"));
    }

    #[test]
    fn title_uses_question_info_stem() {
        let json = r#"{
            "structureType": "biaoti",
            "questionInfo": {"stem": "<span>Section One</span>"},
            "questionIndex": 1
        }"#;

        let output = convert_json(json).unwrap();
        assert!(output.contains("type: \"title\""));
        assert!(output.contains("content: [#latex(`\\rm{Section}`) #latex(`\\rm{One}`)]"));
    }

    #[test]
    fn accepts_string_and_empty_difficulty() {
        let string_value: Record =
            parse_record(r#"{"structureType":"judge","difficulty":"3"}"#).unwrap();
        assert_eq!(string_value.difficulty, 3);

        let empty_value: Record =
            parse_record(r#"{"structureType":"judge","difficulty":""}"#).unwrap();
        assert_eq!(empty_value.difficulty, 0);
    }

    #[test]
    fn rejects_nested_composite_questions() {
        let json = r#"{
            "structureType": "fuhe",
            "smallQuestions": [{
                "structureType": "fuhe",
                "smallQuestions": [{"structureType": "zhuguan"}]
            }]
        }"#;

        assert!(convert_json(json).is_err());
    }

    #[test]
    fn renders_subjective_answer_and_analysis() {
        let json = r#"{
            "structureType": "zhuguan",
            "businessType": "CSX-JIEDA",
            "difficulty": 4,
            "questionInfo": {
                "stem": "<p>Solve $x+1=2$.</p>",
                "answer": "<p>$x=1$</p>",
                "analysis": "<p>Subtract $1$ from both sides.</p>"
            }
        }"#;

        let output = convert_json(json).unwrap();
        assert!(output.contains("type: \"subj\""));
        assert!(output.contains("answer: ([#latex(`x=1`)],)"));
        assert!(output.contains("analysis: ["));
        assert!(output.contains("#latex(`1`)"));
    }

    #[test]
    fn renders_composite_subjective_answer_and_analysis() {
        let json = r#"{
            "structureType": "fuhe",
            "questionInfo": {"stem": "<p>Material</p>"},
            "smallQuestions": [{
                "structureType": "fuhe_zhuguan",
                "businessType": "CSX-JST",
                "questionInfo": {
                    "stem": "<p>Child</p>",
                    "answer": "<p>$2$</p>",
                    "analysis": "<p>Child analysis</p>"
                }
            }]
        }"#;

        let output = convert_json(json).unwrap();
        assert!(output.contains("type: \"composite\""));
        assert!(output.contains("children: ("));
        assert!(output.contains("type: \"subj\""));
        assert!(output.contains("answer: ([#latex(`2`)],)"));
        assert!(output.contains("analysis: ["));
        assert!(output.contains("smallQuestions: (("));
        assert!(!output.contains("smallQuestions: (\n"));
    }

    #[test]
    fn converts_top_level_page_array() {
        let page = r#"[
            {"structureType":"judge","questionIndex":2,"questionInfo":{"stem":"<p>Second</p>"}},
            {"structureType":"biaoti","questionIndex":1,"questionInfo":{"stem":"<p>Title</p>"}}
        ]"#;

        let output = convert_page_json(page).unwrap();
        assert!(output.find("type: \"judge\"").unwrap() < output.find("type: \"title\"").unwrap());
        assert_eq!(output.matches("questionIndex:").count(), 2);
    }

    #[test]
    fn preserves_title_position_in_page_array() {
        let page = r#"[
            {"structureType":"judge","questionInfo":{"stem":"<p>Before</p>"}},
            {"structureType":"biaoti","questionInfo":{"stem":"<p>Middle title</p>"}},
            {"structureType":"judge","questionInfo":{"stem":"<p>After</p>"}}
        ]"#;

        let output = convert_page_json(page).unwrap();
        let before = output.find("#latex(`\\rm{Before}`)").unwrap();
        let title = output.find("type: \"title\"").unwrap();
        let after = output.find("#latex(`\\rm{After}`)").unwrap();
        assert!(before < title && title < after);
    }

    #[test]
    fn converts_wrapped_page_object() {
        let page = r#"{"data":{"records":[
            {"structureType":"judge","questionInfo":{"stem":"<p>Question</p>"}}
        ]}}"#;

        assert!(convert_page_json(page).unwrap().contains("type: \"judge\""));
    }

    #[test]
    fn converts_questions_page_object() {
        let page = r#"{
            "questions": [
                {
                    "structureType": "biaoti",
                    "questionInfo": {"stem": "<span>111</span>"},
                    "questionIndex": 1,
                    "questionId": "3635889599578992640"
                },
                {
                    "structureType": "judge",
                    "questionInfo": {"stem": "<p>Question</p>"},
                    "questionIndex": 2
                }
            ]
        }"#;

        let output = convert_page_json(page).unwrap();
        assert!(output.find("type: \"title\"").unwrap() < output.find("type: \"judge\"").unwrap());
        assert!(output.contains("questionId: \"3635889599578992640\""));
    }

    #[test]
    fn converts_nested_questions_page_object() {
        let page = r#"{
            "data": {
                "questions": [
                    {
                        "structureType": "biaoti",
                        "questionInfo": {"stem": "<span>111</span>"}
                    }
                ]
            }
        }"#;

        assert!(convert_page_json(page).unwrap().contains("type: \"title\""));
    }

    #[test]
    fn converts_question_datas_script() {
        let page = r#"var questionDatas = [
            {"structureType":"judge","questionInfo":{"stem":"<p>Question</p>"},},
        ];
        sendQuestionsSequentially(questionDatas);"#;

        assert!(convert_page_json(page).unwrap().contains("type: \"judge\""));
    }
}
