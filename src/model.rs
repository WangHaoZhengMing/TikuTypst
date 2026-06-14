use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub question_content: String,
    #[serde(default)]
    pub question_index: Index,
    #[serde(default)]
    pub business_type: String,
    #[serde(default)]
    pub structure_type: String,
    #[serde(default, deserialize_with = "deserialize_difficulty")]
    pub difficulty: i64,
    #[serde(default)]
    pub question_info: QuestionInfo,
    #[serde(default)]
    pub small_questions: Vec<Record>,
    #[serde(default)]
    pub question_source: Value,
    #[serde(default)]
    pub question_type: Value,
    #[serde(default)]
    pub relation_type: Value,
    #[serde(default)]
    pub parent_id: Value,
    #[serde(default)]
    pub question_id: Value,
    #[serde(default)]
    pub paper_id: Value,
    #[serde(default)]
    pub sys_code: Value,
    #[serde(default)]
    pub standard_flag: Value,
    #[serde(default)]
    pub input_type: Value,
    #[serde(default)]
    pub add_flag: Value,
    #[serde(default)]
    pub flag_use: Value,
    #[serde(default)]
    pub knw_infos: Vec<KnowledgeInfo>,
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

    pub fn as_value(&self) -> Value {
        match self {
            Self::Number(value) => Value::from(*value),
            Self::Text(value) => Value::from(value.clone()),
            Self::Missing => Value::Null,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DifficultyInput {
    Integer(i64),
    String(String),
}

fn deserialize_difficulty<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<DifficultyInput>::deserialize(deserializer)?;
    match value {
        Some(DifficultyInput::Integer(value)) => Ok(value),
        Some(DifficultyInput::String(value)) if value.trim().is_empty() => Ok(0),
        Some(DifficultyInput::String(value)) => {
            value.parse::<i64>().map_err(serde::de::Error::custom)
        }
        None => Ok(0),
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionItem {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub html_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeInfo {
    #[serde(default)]
    pub knw_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QuestionKind {
    Title,
    SingleChoice,
    MultipleChoice,
    FillBlank,
    Judge,
    Subjective,
    Composite,
    Completion,
}

#[derive(Debug)]
pub(crate) struct QuestionFields {
    pub kind: QuestionKind,
    pub stem: String,
    pub answer: Value,
    pub analysis: String,
    pub options: Vec<OptionItem>,
    pub difficulty: i64,
    pub business_type: String,
    pub question_index: Option<i64>,
    pub children: Vec<QuestionFields>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone)]
pub(crate) struct Metadata {
    pub question_source: Value,
    pub question_type: Value,
    pub relation_type: Value,
    pub parent_id: Option<Value>,
    pub question_id: Option<Value>,
    pub paper_id: Value,
    pub question_index: Value,
    pub sys_code: Value,
    pub standard_flag: Value,
    pub input_type: Value,
    pub add_flag: Value,
    pub flag_use: Value,
    pub knw_names: Vec<String>,
    pub children: Vec<Metadata>,
}
