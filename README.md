# TikuTypst

将题库 JSON 转换为 `json/dsl.typ` 所描述的 Typst DSL。

## 使用

转换单个文件并输出到终端：

```powershell
cargo run --bin TikuTypst -- json/sc.json
```

转换目录中的全部 `.json` 文件：

```powershell
cargo run --bin TikuTypst -- json -o output.typ
```

目录模式下，标题排在最前，其余题目按 `questionIndex` 排序。无法识别的题型、
缺少横线的填空题、嵌套复合题等不符合规则的输入会返回错误，不会静默生成错误 DSL。

## 已支持题型

- 标题：`title`
- 单选题：`sc`
- 多选题：`mc`
- 填空题：`fb`
- 判断题：`judge`
- 主观题：`subj`
- 复合题：`composite`
- 阅读还原/完形：`completion`
