use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tiku_typst::{parse_record, record_sort_key, render_record};

fn main() {
    if let Err(error) = run() {
        eprintln!("错误: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let (input, output) = parse_args()?;
    let result = if input.is_dir() {
        convert_directory(&input)?
    } else {
        convert_file(&input)?
    };

    if let Some(output) = output {
        fs::write(&output, result)
            .with_context(|| format!("无法写入输出文件 {}", output.display()))?;
    } else {
        println!("{result}");
    }
    Ok(())
}

fn parse_args() -> Result<(PathBuf, Option<PathBuf>)> {
    let mut args = env::args_os().skip(1);
    let Some(input) = args.next() else {
        print_usage();
        bail!("缺少 JSON 文件或目录参数");
    };
    if input == "-h" || input == "--help" {
        print_usage();
        std::process::exit(0);
    }

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
    Ok((PathBuf::from(input), output))
}

fn print_usage() {
    eprintln!("用法: TikuTypst <JSON 文件或目录> [-o OUTPUT.typ]");
}

fn convert_file(path: &Path) -> Result<String> {
    let json =
        fs::read_to_string(path).with_context(|| format!("无法读取输入文件 {}", path.display()))?;
    let record = parse_record(&json).with_context(|| format!("解析 {} 失败", path.display()))?;
    render_record(&record).with_context(|| format!("转换 {} 失败", path.display()))
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

    let mut rendered = Vec::with_capacity(records.len());
    for (file_path, record) in records {
        rendered.push(
            render_record(&record).with_context(|| format!("转换 {} 失败", file_path.display()))?,
        );
    }
    Ok(rendered.join("\n\n"))
}
