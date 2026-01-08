# Auto-NVM Rust 分阶段实现计划

## 项目概述
创建一个跨平台的 Node.js 版本自动切换器，支持 bash、zsh、fish、powershell，在用户 cd 到包含 .nvmrc 文件的目录时自动切换 Node.js 版本。

**技术栈**: Rust + 最小化 Shell 集成脚本
**性能目标**: < 500ms 切换延迟
**分发策略**: 灵活支持多种方式 (Cargo/预编译二进制/包管理器)

---

## 第一阶段：Rust 项目初始化 (MVP)
**目标**: 建立 Cargo 项目基础架构和核心功能
**预期时间**: 1-2 天
**成功标准**: 基本的 .nvmrc 检测和版本切换功能完成

### 任务清单

#### 1.1 Cargo 项目初始化 🔄
- [x] 创建 Rust 项目结构 ✅ 已完成
  ```bash
  cargo init  # 已执行，创建了基础项目结构
  # 待完成：mkdir -p src/{config,nvmrc,nvm,cache} shell-integration/{bash,zsh,fish,powershell} scripts
  ```
- [ ] 配置 `Cargo.toml` 依赖
  ```toml
  [dependencies]
  clap = { version = "4.0", features = ["derive"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  anyhow = "1.0"
  dirs = "5.0"
  which = "4.0"
  ```

#### 1.2 核心 Rust 模块 ⏳
- [ ] `src/main.rs` - 程序入口和 CLI 接口
  - 使用 clap 定义命令行参数
  - 主要命令: `auto-nvm check`, `auto-nvm setup`, `auto-nvm switch`
  - 错误处理和用户友好的输出

- [ ] `src/nvmrc/mod.rs` - .nvmrc 文件处理模块
  - 向上递归查找 .nvmrc 文件 (`std::path::Path`)
  - 解析版本号格式 (支持语义化版本)
  - 验证版本号有效性
  - 优雅的错误处理 (`anyhow::Result`)

- [ ] `src/nvm/mod.rs` - NVM 命令抽象层
  - 检测 NVM 类型 (Unix nvm vs Windows nvm-windows)
  - 抽象 NVM 命令接口
  - 执行版本切换命令
  - 验证切换结果

- [ ] `src/config/mod.rs` - 配置管理模块
  - 使用 `serde` 处理配置序列化
  - 支持 TOML/JSON 配置文件
  - 环境变量覆盖
  - 默认配置值

#### 1.3 基础命令实现 ⏳
- [ ] `check` 命令 - 检查当前目录
  - 查找 .nvmrc 文件
  - 显示当前和目标 Node.js 版本
  - 输出是否需要切换

- [ ] `switch` 命令 - 执行版本切换
  - 调用 NVM 命令切换版本
  - 验证切换成功
  - 输出切换结果

#### 1.4 基础文档 ⏳
- [ ] `README.md` - 项目介绍
  - 项目描述和 Rust 实现优势
  - 安装方式 (cargo install)
  - 基本使用方法

### 测试验证
- [ ] 创建测试 .nvmrc 文件
- [ ] 验证 `auto-nvm check` 命令
- [ ] 验证 `auto-nvm switch` 命令
- [ ] 单元测试核心模块 (`cargo test`)

### 阶段产出
✅ 基础的 Rust CLI 工具，支持手动版本切换

---

## 第二阶段：Shell 集成系统
**目标**: 实现自动 Shell 集成和 cd 钩子
**预期时间**: 2-3 天
**成功标准**: 用户 cd 到目录时自动切换 Node.js 版本

### 任务清单

#### 2.1 Shell 检测和配置 ⏳
- [ ] `setup` 命令实现 - 自动配置 Shell 集成
  - 检测用户当前使用的 Shell (`$SHELL` 环境变量)
  - 自动生成对应的 Shell 集成脚本
  - 修改用户的 Shell 配置文件 (`.bashrc`, `.zshrc` 等)
  - 提供手动配置说明

#### 2.2 多 Shell 支持实现 ⏳
- [ ] `shell-integration/bash/auto-nvm.bash` - Bash 支持
  - cd 命令包装器 (`function cd()`)
  - 调用 `auto-nvm check` 并在需要时自动切换
  - 保持原始 cd 功能

- [ ] `shell-integration/zsh/auto-nvm.zsh` - Zsh 支持
  - 使用 `chpwd` 钩子函数
  - Zsh 特定的函数定义
  - 兼容 Oh My Zsh

- [ ] `shell-integration/fish/auto-nvm.fish` - Fish 支持
  - 使用 Fish 的 `cd` 函数包装
  - Fish 特定语法适配
  - 事件驱动机制

- [ ] `shell-integration/powershell/auto-nvm.psm1` - PowerShell 支持
  - PowerShell 模块结构
  - `Set-Location` 函数包装
  - Windows 路径处理

#### 2.3 Rust 端增强 ⏳
- [ ] 添加 `--quiet` 模式 - Shell 集成专用
  - 静默执行，只输出必要信息
  - 优化性能，减少输出延迟
  - 返回简洁的状态码

- [ ] 改进错误处理
  - 区分用户错误和系统错误
  - 提供建设性的错误信息
  - 日志记录 (可选)

### 测试验证
- [ ] 在不同 Shell 中测试 cd 钩子
- [ ] 验证自动版本切换功能
- [ ] 测试 `auto-nvm setup` 命令
- [ ] 跨平台兼容性测试 (macOS/Linux/Windows)

### 阶段产出
✅ 完整的自动切换功能，支持主要 Shell

---

## 第三阶段：性能优化和缓存
**目标**: 实现高性能缓存系统，达到 < 500ms 目标
**预期时间**: 1-2 天
**成功标准**: 切换延迟 < 500ms，智能缓存工作正常

### 任务清单

#### 3.1 缓存系统实现 ⏳
- [ ] `src/cache/mod.rs` - 缓存机制实现
  - .nvmrc 文件位置缓存 (目录 -> .nvmrc 路径映射)
  - 当前版本信息缓存
  - 基于文件修改时间的 TTL 机制
  - 使用 `serde` 序列化缓存数据到 JSON

#### 3.2 性能优化策略 ⏳
- [ ] 智能跳过机制
  - 检测当前目录是否与上次相同
  - 缓存当前 Node.js 版本，避免重复查询
  - 只在 .nvmrc 内容变化时重新解析

- [ ] 目录排除优化
  - 内置系统目录排除列表 (`/tmp`, `/var`, `/proc` 等)
  - 用户自定义排除配置
  - 快速路径匹配算法

- [ ] Rust 性能优化
  - 使用 `std::collections::HashMap` 优化查找
  - 减少不必要的文件系统操作
  - 优化字符串处理和路径操作

#### 3.3 高级配置选项 ⏳
- [ ] 扩展配置文件功能
  - 缓存 TTL 配置
  - 排除目录列表
  - 日志级别设置
  - 自动安装缺失版本选项

- [ ] 用户体验优化
  - 版本切换确认提示 (可配置)
  - 自定义成功/错误消息
  - 进度指示器 (长时间操作)

### 测试验证
- [ ] 性能基准测试 (`cargo bench`)
- [ ] 缓存有效性测试
- [ ] 大量目录切换压力测试
- [ ] 内存使用分析 (`valgrind` 或 `heaptrack`)

### 阶段产出
✅ 高性能的生产就绪版本，满足 < 500ms 性能目标

---

## 第四阶段：测试、文档和发布准备
**目标**: 完善测试覆盖、文档和多种分发方式
**预期时间**: 2-3 天
**成功标准**: 完整的测试覆盖，支持多种分发方式

### 任务清单

#### 4.1 Rust 测试套件 ⏳
- [ ] 单元测试 (`src/*/mod.rs` 中的 `#[cfg(test)]`)
  - .nvmrc 解析功能测试
  - 配置管理测试
  - 缓存系统测试
  - 错误处理测试

- [ ] 集成测试 (`tests/` 目录)
  - CLI 命令集成测试
  - Shell 集成端到端测试
  - 跨平台兼容性测试

- [ ] 性能测试 (`benches/` 目录)
  - 使用 `criterion` 进行性能基准测试
  - 缓存命中率测试
  - 内存使用测试

#### 4.2 分发方式准备 ⏳
- [ ] Cargo 发布准备
  - 完善 `Cargo.toml` 元数据
  - 准备 crates.io 发布
  - 设置 CI/CD 自动发布

- [ ] 预编译二进制支持
  - GitHub Actions 交叉编译
  - 支持 `cargo-binstall`
  - 多平台二进制发布

- [ ] 包管理器支持 (可选)
  - Homebrew Formula 模板
  - Scoop manifest 模板
  - 安装脚本模板

#### 4.3 文档和发布 ⏳
- [ ] `README.md` - 完整项目文档
  - Rust 实现优势说明
  - 多种安装方式 (cargo install/预编译/包管理器)
  - 使用示例和配置说明

- [ ] `docs/installation.md` - 详细安装指南
  - Rust 环境安装指南
  - 各平台安装方式
  - 故障排除指南

- [ ] 发布准备
  - `LICENSE` 文件 (MIT)
  - `CONTRIBUTING.md` - 贡献指南
  - `CHANGELOG.md` - 版本变更记录

### 测试验证
- [ ] 完整的回归测试 (`cargo test`)
- [ ] 文档准确性验证
- [ ] 多平台用户体验测试
- [ ] 分发流程测试 (cargo publish/GitHub releases)

### 阶段产出
✅ 可发布的完整 Rust 项目，支持多种分发方式

---

## Rust 项目结构

```
auto-nvm/
├── Cargo.toml                 # 项目配置和依赖
├── src/
│   ├── main.rs               # CLI 入口点
│   ├── config/
│   │   └── mod.rs           # 配置管理模块
│   ├── nvmrc/
│   │   └── mod.rs           # .nvmrc 文件处理
│   ├── nvm/
│   │   └── mod.rs           # NVM 抽象层
│   └── cache/
│       └── mod.rs           # 缓存系统
├── shell-integration/         # Shell 集成脚本
│   ├── bash/auto-nvm.bash
│   ├── zsh/auto-nvm.zsh
│   ├── fish/auto-nvm.fish
│   └── powershell/auto-nvm.psm1
├── tests/                    # 集成测试
├── benches/                  # 性能测试
├── scripts/                  # 安装脚本 (可选)
└── docs/                     # 文档
```

## 实施建议

### Rust 开发环境准备
- 安装 Rust 工具链 (`rustup`)
- 配置开发工具 (VS Code + rust-analyzer)
- 确保已安装 nvm 或 nvm-windows 用于测试
- 准备多种 Shell 环境进行测试

### 质量控制
- 使用 `cargo fmt` 保持代码格式一致
- 使用 `cargo clippy` 进行代码质量检查
- 每个阶段完成后运行 `cargo test`
- 添加充分的错误处理和文档注释

### 版本控制
- 每个阶段完成后创建 git tag
- 保持清晰的 commit 历史
- 及时更新文档和 CHANGELOG

### 分发策略灵活性
- **开发阶段**: 专注于核心功能实现
- **测试阶段**: 准备多种分发方式的基础设施
- **发布阶段**: 根据用户反馈选择主要分发渠道
- **可选方式**: Cargo install, 预编译二进制, 包管理器, 安装脚本

### 性能目标
- 目标延迟: < 500ms (Rust 性能优势使这很容易达到)
- 内存使用: 最小化运行时内存占用
- 启动时间: 快速冷启动
- 缓存效率: 智能缓存减少重复计算