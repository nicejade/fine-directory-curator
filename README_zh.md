<p align="center">
  <a href="https://fine.niceshare.site/projects/fine-directory-curator/" target="_blank">
    <img width="120"
    src="https://lovejade.oss-cn-shenzhen.aliyuncs.com/fine-directory-curator-logo.png">
  </a>
</p>

<h1 align="center"><a href="https://fine.niceshare.site/projects/fine-directory-curator">Fine Directory Curator (fdc)</a></h1>

<div align="center">
  <strong>
  🗂️ 一个快速、优雅的 Rust 命令行工具，用于让 macOS 与 Linux 文件夹保持整洁（计划支持 Windows）。
它会按 <code>年份</code> 和 <code>类型</code> 对源目录的一级项进行归类，避免覆盖冲突，保留顶层结构，并允许在进行任何破坏性更改前先<code>预览</code>。
  </strong>

  <p dir="auto">
    <img src="https://lovejade.oss-cn-shenzhen.aliyuncs.com/fine-directory-curator.png" alt="Fine Directory Curator (fdc)" data-canonical-src="https://lovejade.oss-cn-shenzhen.aliyuncs.com/fine-directory-curator.png" style="max-width: 100%;">
  </p>
</div>

> TL;DR: `cargo install fine-directory-curator` → `fdc --dry-run` → 审阅 → `fdc`
> 无需折腾，不会重复覆盖，也不用再问“我的下载去哪儿了？”

---

## 目录
- [为什么要用 fdc？](#为什么要用-fdc)
- [安装](#安装)
- [快速开始](#快速开始)
- [CLI 参考](#cli-参考)
  - [命令](#命令)
  - [全局参数](#全局参数)
  - [示例](#示例)
- [配置](#配置)
  - [文件布局与规则](#文件布局与规则)
  - [扩展名分类](#扩展名分类)
- [行为与保证](#行为与保证)
- [操作系统支持](#操作系统支持)
- [开发](#开发)
- [故障排查与常见问题](#故障排查与常见问题)
- [路线图](#路线图)
- [相关链接](#相关链接)
- [许可证](#许可证)

---

## 为什么要用 fdc？

因为现实是：下载文件夹会被塞满、发生命名冲突、并且递归工具有风险。
**fdc** 故意有明确设计取向：
- **浅且安全**：仅处理源目录的**第一层**内容。目录作为整体移动，不会递归深入修改。
- **年份优先**：时间是“该文件放在哪儿”的最佳主键。
- **不覆盖**：如果冲突则生成 `name (1).ext`, `name (2).ext` 等，而不是覆盖已有文件。
- **先预览**：使用 `--dry-run` 可查看将要发生的改动。
- **快速且跨平台**：用 Rust 编写，在 macOS 与 Linux 上运行良好。

---

## 安装

### 从 crates.io（推荐）

```bash
cargo install fine-directory-curator
# 可执行文件将安装在 PATH 中，名称为：
fdc
```

> 注意：包名为 `fine-directory-curator`，二进制可执行文件名为 `fdc`。

### 从源代码构建

```bash
# 在仓库根目录下
cargo build --release
# 二进制文件路径：
./target/release/fdc
```

---

## 快速开始

```bash
# 查看帮助
fdc --help

# 使用默认（source: ~/Downloads, target: ~/Documents/Matrixs）
fdc

# 仅预览（不做任何修改）
fdc --dry-run

# 指定源与目标
fdc -s ~/Downloads -t ~/Documents/Matrixs

# 仅在配置中设置 source（不执行移动）
fdc -s ~/Downloads

# 详细日志（可叠加）
fdc -v
fdc -vv
```

---

## CLI 参考

### 命令

| 命令 | 别名 | 用途 | 说明 |
|---|---|---|---|
| `fdc` | — | 使用当前配置（或默认）运行整理 | 可配合参数自定义运行 |
| `fdc init-config` | — | 创建默认配置文件 | 不覆盖已存在的文件，会提示 |
| `fdc config` | — | 打印解析后的配置 | 便于调试 |
| `fdc set-source <PATH>` | — | 在配置中设置源目录 | 仅更新配置文件 |
| `fdc set-target <PATH>` | — | 在配置中设置目标根目录 | 仅更新配置文件 |
| `fdc --help` | `-h` | 显示帮助 | — |
| `fdc --version` | `-V` | 显示版本号 | 遵循 SemVer |

### 全局参数

| 参数 | 别名 | 类型 | 默认 | 含义 | 示例 |
|---|---|---|---|---|---|
| `--dry-run` | — | bool | `false` | 仅计划；打印将要执行的动作，不修改文件系统 | `fdc --dry-run` |
| `--source <DIR>` | `-s` | 路径 | `~/Downloads` | 覆盖源目录 | `fdc -s ~/Desktop` |
| `--target <DIR>` | `-t` | 路径 | `~/Documents/Matrixs` | 覆盖目标根目录 | `fdc -t ~/Archive` |
| `--verbose` | `-v` | 计数 | `0` | 增加日志详细度（可叠加） | `fdc -vv` |
| `--help` | `-h` | — | — | 显示用法 | `fdc -h` |
| `--version` | `-V` | — | — | 显示版本 | `fdc -V` |

> **智能参数行为**：当只提供 `-s`（未提供 `-t`, `--dry-run` 或 `-v`）时，fdc 只会更新配置文件并不会执行文件操作。

### 示例

| 目标 | 命令 |
|---|---|
| 预览将要做的清理 | `fdc --dry-run` |
| 将当前 Downloads 整理到归档 | `fdc` |
| 使用自定义目标根目录 | `fdc -t ~/Documents/Matrixs` |
| 从不同源执行整理 | `fdc -s ~/Desktop` |
| 仅在配置中设置源 | `fdc -s ~/Desktop` |
| 以命令形式设置源 | `fdc set-source ~/Desktop` |
| 在配置中设置目标 | `fdc set-target ~/Archive` |
| 打开详细日志 | `fdc -vv` |
| 初始化（或重建）配置文件 | `fdc init-config` ; `fdc config` |

---

## 配置

首次运行时，fdc 会写入默认配置到：

- **macOS**: `~/Library/Application Support/fine-directory-curator/config.toml`  
- **Linux**: `~/.config/fine-directory-curator/config.toml`

> 若想提前创建配置，建议使用 `fdc init-config`。

### 示例 `config.toml`
```toml
source_dir = "~/Downloads"
target_dir = "~/Documents/Matrixs"

[sort_rule]
# 排序优先级，从左到右
order = ["year", "category"]

# 可选：将特定扩展映射到自定义分类
[extension_overrides]
# xmind = "mindmaps"
# heic  = "images"
```

### 文件布局与规则

- **年份** 放在首位，来源顺序为：
  1) 文件的 **创建时间**；
  2) 若不可用则回退到 **修改时间**；
  3) 若都不可用则使用 **当前年份**。
- **分类** 放在第二位（见上文扩展名分类）。
- **示例结构**：
  ```
  <target_dir>/
    2025/
      images/
      documents/
      videos/
      ...
  ```

### 扩展名分类
fdc 将扩展名（忽略大小写）映射到如下桶（bucket）：

- `images`, `pdfs`, `videos`, `audio`, `archives`,  
  `documents`, `spreadsheets`, `presentations`,  
  `code`, `design`, `mindmaps`, `executables`,  
  `installers`, `fonts`, `others`, `directory`

> 注意：目录不会被递归遍历；它们被视为单个项并移动到 `directory/`。

---

## 行为与保证

- **不覆盖策略**：若目标路径已存在，fdc 会生成 `name (1).ext`, `name (2).ext` 等直到找到空位。
- **浅层操作**：仅处理 `source_dir` 的第一层条目。
- **跨卷支持**：跨文件系统移动采用 **复制 + 删除** 的方式。
- **幂等性（近似）**：再次运行不会在已存在位置重复产生文件，因为使用了非覆盖命名规则。
- **安全优先**：新的设置建议先从 `--dry-run` 开始。

---

## 操作系统支持

| 操作系统 | 状态 | 备注 |
|---|---|---|
| **macOS** | ✅ 支持 | 在 Apple Silicon 与 Intel 上已测试 |
| **Linux** | ✅ 支持 | 默认使用 XDG 配置路径 |
| **Windows** | 🛤️ 规划中 | 需处理路径语义与元数据 |

> macOS 用户：若整理的目录在家目录以外，可能需要为 Terminal（或所用 shell）授予 **完全磁盘访问**。

---

## 开发

精简的开发流程，便于快速迭代并保持质量。

### 前置条件
- Rust 1.75+（使用 `rustup`）
- `cargo fmt`、`clippy` 和 `cargo test`

### 常用任务
```bash
# 格式化
cargo fmt

# 严格 lint
cargo clippy -- -D warnings

# 单元与集成测试
cargo test

# release 构建
cargo build --release
```

### 推荐的开发体验（可选）
- 使用 `justfile` 或 `Makefile` 组织常用命令：
  ```makefile
  build: ; cargo build --release
  check: ; cargo fmt --check && cargo clippy -- -D warnings && cargo test
  release: check build
  ```
- 添加 pre-commit 钩子以保证提交质量：
  ```bash
  # .git/hooks/pre-commit
  cargo fmt -- --check &&
  cargo clippy -- -D warnings &&
  cargo test
  ```
- 使用 `cargo-release` 来管理标签、变更日志与 `cargo publish`。

---

## 故障排查与常见问题

**问：什么都没有移动，为什么？**  
答：使用 `-vv` 查看 fdc 的决策过程。确保源目录在顶层含有可处理的条目；fdc 不会递归子目录。

**问：为什么把“年份”放在优先？**  
答：时间在各种文件类型与工作流中通常是最普适的键，能让归档易于浏览。

**问：可以自定义分类吗？**  
答：可以，在 `config.toml` 中使用 `[extension_overrides]`。未知扩展会被放到 `others/`。

**问：`--dry-run` 真的是非破坏性的吗？**  
答：是的。它只打印计划；不会对文件系统做写入操作。

**问：如何处理重复项？**  
答：fdc 永远不会覆盖，而是选择 `name (N).ext` 的下一个可用名称。

**问：fdc 会改变文件元数据吗？**  
答：移动通常会保留元数据；跨设备复制 + 删除 时可能因底层文件系统差异而有所不同。fdc 使用原始时间戳来判断归类，而不会强制改写时间戳。

---

## 路线图

- ✅ 已实现 macOS 与 Linux 的功能对齐
- ⏭️ 计划支持 Windows（路径、时间戳、替代数据流）
- ⏭️ 可配置的命名模板（例如 `YYYY/category` vs `category/YYYY`）
- ⏭️ 忽略/包含模式（如 `.fdcignore`）
- ⏭️ 汇总报告输出（`--report json|markdown`）
- ⏭️ 将 dry-run 输出保存为文件 (`--plan <path>`) 的功能

> 有想法或意见？欢迎提交 issue 或 PR —— 我们一起把文件系统变得更有序。

---

## 相关链接

- [逍遥自在轩](https://www.niceshare.site/)
- [清风明月轩](https://www.lovejade.cn/)
- [晚晴幽草轩](https://www.jeffjade.com/)
- [缘知随心庭](https://fine.niceshare.site/)
- [玉桃文飨轩](https://share.lovejade.cn/)
- [倾城之链](https://site.lovejade.cn/)
- [曼妙句子](https://read.lovejade.cn/)
- [SegmentFault](https://segmentfault.com/u/jeffjade)
- [X | MarshalXuan](https://x.com/MarshalXuan)
- [@MarshalXuan](https://www.youtube.com/@MarshalXuan)

---

## 许可证

[MIT](http://opensource.org/licenses/MIT) © 2025–至今, [逍遥自在轩](https://www.niceshare.site/)。