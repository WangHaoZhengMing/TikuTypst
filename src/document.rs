use crate::typst_string;

pub(crate) fn render_document(items: &[String]) -> String {
    let paper_data = render_array(items.iter().map(String::as_str));
    format!("#let paper_data = {paper_data}\n\n#paper_data")
}

pub(crate) fn render_array<'a>(items: impl Iterator<Item = &'a str>) -> String {
    let items = items
        .map(|item| indent(item, "  "))
        .map(|item| format!("{item},"))
        .collect::<Vec<_>>();
    if items.is_empty() {
        "()".to_owned()
    } else {
        format!("(\n{}\n)", items.join("\n"))
    }
}

pub(crate) fn render_metadata_string_array(values: &[String]) -> String {
    if values.is_empty() {
        return "()".to_owned();
    }
    let values = values
        .iter()
        .map(|value| typst_string(value))
        .collect::<Vec<_>>();
    format!("({},)", values.join(", "))
}

pub(crate) fn indent(value: &str, prefix: &str) -> String {
    value
        .lines()
        .map(|line| format!("{prefix}{line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
