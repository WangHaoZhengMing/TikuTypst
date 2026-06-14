use crate::typst_string;

pub(crate) fn render_document(items: &[(String, String)]) -> String {
    let forrender = render_array(items.iter().map(|(render, _)| render.as_str()));
    let metadata = render_array(items.iter().map(|(_, metadata)| metadata.as_str()));
    format!("#let forrender = {forrender}\n\n#let metadata = {metadata}\n\n#(forrender, metadata)")
}

fn render_array<'a>(items: impl Iterator<Item = &'a str>) -> String {
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
