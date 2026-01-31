# Auto-NVM

[![CI](https://github.com/zerosrat/auto-nvm/workflows/CI/badge.svg)](https://github.com/zerosrat/auto-nvm/actions)
[![Release](https://github.com/zerosrat/auto-nvm/workflows/Release/badge.svg)](https://github.com/zerosrat/auto-nvm/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](./README.md) | [中文](./README.zh-CN.md)

**Unix Node.js 版本自动切换器，支持多种 Shell**

Auto-NVM 会在你进入包含 `.nvmrc` 文件的目录时自动切换 Node.js 版本。使用 Rust 构建以获得速度和可靠性，支持 Bash、Zsh 和 Fish，兼容 Linux 和 macOS。

## ✨ 特性

- 🚀 **自动版本切换** - 无需手动干预
- 🔧 **多 Shell 支持** - 支持 Bash、Zsh 和 Fish
- 🌍 **Unix 平台** - Linux 和 macOS
- ⚡ **高性能** - 使用 Rust 构建，切换延迟 < 500ms
- 🎯 **零配置** - 安装后开箱即用
- 🛡️ **安全可靠** - 全面的错误处理和回滚机制
- 📦 **简易安装** - 一条命令安装并配置所有内容

## 🚀 快速开始

### 一条命令安装

**Unix (Linux/macOS):**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

就这样！安装脚本会自动：
1. 下载适合你平台的二进制文件
2. 安装到 PATH
3. 自动配置 Shell 集成
4. 设置自动版本切换

### 验证安装

```bash
auto-nvm --version
```

测试自动切换：
```bash
echo "18.17.0" > .nvmrc
cd .  # 触发自动切换
node --version  # 应该显示 v18.17.0
```

## 📖 工作原理

1. **在项目目录中创建 `.nvmrc` 文件**：
   ```bash
   echo "18.17.0" > .nvmrc
   ```

2. **进入目录** - auto-nvm 自动检测并切换：
   ```bash
   cd my-project/  # 自动切换到 Node.js 18.17.0
   ```

3. **支持的版本格式**：
   - 具体版本号：`18.17.0`、`16.20.1`
   - 语义版本：`18`、`16.20`
   - 别名：`lts`、`stable`、`latest`

## 🔧 安装方式

### 1. 通用安装脚本（推荐）

**特性：**
- ✅ 自动平台检测
- ✅ 二进制安装 + Shell 配置
- ✅ PATH 设置
- ✅ 支持所有平台

**Unix:**
```bash
curl -fsSL https://raw.githubusercontent.com/zerosrat/auto-nvm/main/install.sh | bash
```

### 2. 从源码构建

```bash
git clone https://github.com/zerosrat/auto-nvm.git
cd auto-nvm
cargo build --release
cp target/release/auto-nvm ~/.local/bin/
auto-nvm setup
```

## 🐚 Shell 集成

Auto-NVM 通过包装 `cd` 命令与你的 Shell 集成。安装后，会自动在以下 Shell 中工作：

### Bash
集成配置添加到 `~/.bashrc` 或 `~/.bash_profile`

### Zsh
集成配置添加到 `~/.zshrc`

### Fish
集成配置添加到 `~/.config/fish/config.fish`

### 手动 Shell 配置

如果自动设置失败，你可以手动配置：

```bash
auto-nvm setup  # 为当前 Shell 配置
```

或指定具体的 Shell：
```bash
SHELL=/bin/zsh auto-nvm setup  # 为 Zsh 配置
```

## 📋 命令

### `auto-nvm check`
检查当前目录的 `.nvmrc` 并显示版本信息：
```bash
auto-nvm check
# 输出: Found .nvmrc with version: 18.17.0
#       Current Node.js version: v16.20.1
#       Switch needed: yes
```

### `auto-nvm switch`
手动切换到 `.nvmrc` 版本：
```bash
auto-nvm switch
# 输出: Switched to Node.js v18.17.0
```

### `auto-nvm setup`
配置 Shell 集成：
```bash
auto-nvm setup
# 输出: Shell integration configured for Zsh
#       Added auto-nvm configuration to ~/.zshrc
```

### `auto-nvm uninstall`
移除 Shell 集成（保留二进制文件）：
```bash
sudo auto-nvm uninstall
# 输出: Removed auto-nvm integration from ~/.zshrc
```

## 🔍 故障排除

### 自动切换不工作
1. **验证安装**：`auto-nvm --version`
2. **检查 Shell 集成**：查看你的 Shell RC 文件中是否有 auto-nvm 配置
3. **重新运行设置**：`auto-nvm setup`
4. **重启 Shell**：打开新终端或执行 `source ~/.bashrc`

### 命令未找到
1. **检查 PATH**：`echo $PATH` 应该包含 auto-nvm 安装目录
2. **手动设置 PATH**：在 Shell RC 文件中添加 `export PATH="$HOME/.local/bin:$PATH"`
3. **重新安装**：再次使用安装脚本

### 权限错误
1. **安装到用户目录**：使用 `AUTO_NVM_INSTALL_DIR=~/.local/bin`
2. **检查权限**：确保安装目录可写
3. **使用 sudo**（不推荐）：仅用于系统级安装

### 版本未切换
1. **检查 .nvmrc 格式**：应该只包含版本号
2. **验证 NVM 安装**：`nvm --version`
3. **检查版本是否存在**：`nvm ls-remote | grep <version>`

---

**由 🐟 和 Rust 驱动**
