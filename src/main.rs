use anyhow::{Context, Result, bail};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use tiku_typst::{convert_page_json, parse_record, record_sort_key, render_records};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    Normal,
    Page,
}

struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
    mode: InputMode,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("错误: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = parse_args_from(env::args_os().skip(1))?;
    let result = if args.mode == InputMode::Page {
        convert_page(&args.input)?
    } else if args.input.is_dir() {
        convert_directory(&args.input)?
    } else {
        convert_file(&args.input)?
    };

    if let Some(output) = args.output {
        fs::write(&output, result)
            .with_context(|| format!("无法写入输出文件 {}", output.display()))?;
    } else {
        println!("{result}");
    }
    Ok(())
}

fn parse_args_from(args: impl IntoIterator<Item = OsString>) -> Result<Args> {
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        print_usage();
        bail!("缺少 JSON 文件或目录参数");
    };
    if first == "-h" || first == "--help" {
        print_usage();
        std::process::exit(0);
    }

    let (mode, input) = if first == "-page" || first == "--page" {
        let input = args
            .next()
            .ok_or_else(|| anyhow::anyhow!("{:?} 后缺少页面 JSON 文件路径", first))?;
        (InputMode::Page, input)
    } else {
        (InputMode::Normal, first)
    };

    let mut output = None;
    while let Some(arg) = args.next() {
        if arg == "-o" || arg == "--output" {
            let Some(path) = args.next() else {
                bail!("{:?} 后缺少输出文件路径", arg);
            };
            output = Some(PathBuf::from(path));
        } else {
            bail!("未知参数: {:?}", arg);
        }
    }

    Ok(Args {
        input: PathBuf::from(input),
        output,
        mode,
    })
}

fn print_usage() {
    eprintln!("用法:");
    eprintln!("  TikuTypst <JSON 文件或目录> [-o OUTPUT.typ]");
    eprintln!("  TikuTypst -page <页面 JSON 文件> [-o OUTPUT.typ]");
}

fn convert_file(path: &Path) -> Result<String> {
    let json =
        fs::read_to_string(path).with_context(|| format!("无法读取输入文件 {}", path.display()))?;
    let record = parse_record(&json).with_context(|| format!("解析 {} 失败", path.display()))?;
    render_records(&[record]).with_context(|| format!("转换 {} 失败", path.display()))
}

fn convert_page(path: &Path) -> Result<String> {
    if path.is_dir() {
        bail!("-page 需要文件路径，不能传入目录 {}", path.display());
    }
    let json =
        fs::read_to_string(path).with_context(|| format!("无法读取页面文件 {}", path.display()))?;
    convert_page_json(&json).with_context(|| format!("转换页面 {} 失败", path.display()))
}

fn convert_directory(path: &Path) -> Result<String> {
    let mut records = Vec::new();
    for entry in
        fs::read_dir(path).with_context(|| format!("无法读取输入目录 {}", path.display()))?
    {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let json = fs::read_to_string(&file_path)
            .with_context(|| format!("无法读取输入文件 {}", file_path.display()))?;
        let record =
            parse_record(&json).with_context(|| format!("解析 {} 失败", file_path.display()))?;
        records.push((file_path, record));
    }
    records.sort_by_key(|(_, record)| record_sort_key(record));

    let records = records
        .into_iter()
        .map(|(_, record)| record)
        .collect::<Vec<_>>();
    render_records(&records).with_context(|| format!("转换 {} 失败", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arguments<'a>(values: &'a [&'a str]) -> impl Iterator<Item = OsString> + 'a {
        values.iter().map(OsString::from)
    }

    #[test]
    fn parses_page_argument() {
        let args = parse_args_from(arguments(&["-page", "page.json", "-o", "output.typ"])).unwrap();
        assert_eq!(args.mode, InputMode::Page);
        assert_eq!(args.input, PathBuf::from("page.json"));
        assert_eq!(args.output, Some(PathBuf::from("output.typ")));
    }

    #[test]
    fn keeps_existing_input_mode() {
        let args = parse_args_from(arguments(&["json", "-o", "output.typ"])).unwrap();
        assert_eq!(args.mode, InputMode::Normal);
        assert_eq!(args.input, PathBuf::from("json"));
    }
}
