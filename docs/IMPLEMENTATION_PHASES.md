# Auto-NVM 分阶段实现计划

## 项目概述
创建一个跨平台的 Node.js 版本自动切换器，支持 bash、zsh、fish、powershell，在用户 cd 到包含 .nvmrc 文件的目录时自动切换 Node.js 版本。

---

## 第一阶段：核心功能实现 (MVP)
**目标**: 实现基本的自动切换功能，支持 bash 和 Unix 环境
**预期时间**: 1-2 天
**成功标准**: 在 bash 环境下能够自动检测 .nvmrc 并切换 Node.js 版本

### 任务清单

#### 1.1 项目基础结构 ⏳
- [ ] 创建目录结构
  ```bash
  mkdir -p src/{core,shells/bash,platform,utils} config tests docs
  ```
- [ ] 创建基础文件
  ```bash
  touch README.md LICENSE .gitignore
  ```

#### 1.2 核心功能模块 ⏳
- [ ] `src/platform/detection.sh` - 平台和 NVM 检测
  - 检测操作系统 (macOS/Linux/Windows)
  - 检测 NVM 类型 (nvm/nvm-windows)
  - 检测当前 Node.js 版本

- [ ] `src/core/nvmrc-parser.sh` - .nvmrc 文件处理
  - 向上递归查找 .nvmrc 文件
  - 解析和验证版本号格式
  - 错误处理

- [ ] `src/core/version-manager.sh` - 版本切换逻辑
  - NVM 命令抽象层
  - 版本切换执行
  - 切换结果验证

- [ ] `src/core/config.sh` - 配置管理
  - 加载默认配置
  - 环境变量覆盖
  - 配置验证

- [ ] `src/core/auto-nvm-core.sh` - 主入口逻辑
  - 协调所有模块
  - 主要业务流程
  - 错误处理和日志

#### 1.3 Bash 集成 ⏳
- [ ] `src/shells/bash/auto-nvm.bash` - Bash Shell 集成
  - cd 命令钩子实现
  - 原始 cd 功能保留
  - Shell 初始化函数

#### 1.4 基础配置 ⏳
- [ ] `config/auto-nvm.conf` - 默认配置文件
  - 启用/禁用开关
  - 基础行为配置
  - 日志级别设置

#### 1.5 基础文档 ⏳
- [ ] `README.md` - 项目介绍
  - 项目描述和特性
  - 快速开始指南
  - 基本使用方法

### 测试验证
- [ ] 创建测试 .nvmrc 文件
- [ ] 验证 cd 触发机制
- [ ] 验证版本切换功能
- [ ] 验证错误处理

### 阶段产出
✅ 可在 bash 环境下工作的基本 auto-nvm 版本

---

## 第二阶段：安装部署系统
**目标**: 提供便捷的安装方式和多 Shell 支持
**预期时间**: 2-3 天
**成功标准**: 用户可以一键安装并在多种 Shell 中使用

### 任务清单

#### 2.1 安装脚本 ⏳
- [ ] `install.sh` - Unix/macOS 安装脚本
  - 自动检测用户 Shell 类型
  - 下载和安装文件
  - 自动配置 Shell 集成
  - 权限和路径设置

#### 2.2 多 Shell 支持扩展 ⏳
- [ ] `src/shells/zsh/auto-nvm.zsh` - Zsh 支持
  - chpwd 钩子实现
  - Zsh 特定的自动加载
  - 兼容性处理

- [ ] `src/shells/fish/auto-nvm.fish` - Fish 支持
  - Fish 函数包装器
  - Fish 特定语法适配
  - 事件处理机制

- [ ] `src/shells/powershell/Auto-NVM.psm1` - PowerShell 支持
  - PowerShell 模块结构
  - Set-Location 包装器
  - Windows 特定处理

#### 2.3 用户交互优化 ⏳
- [ ] `src/utils/prompts.sh` - 用户交互
  - 版本安装确认提示
  - 友好的错误信息
  - 进度反馈

- [ ] `src/utils/logging.sh` - 日志系统
  - 分级日志输出
  - 调试信息支持
  - 日志格式标准化

#### 2.4 Windows 支持 ⏳
- [ ] `install.ps1` - Windows PowerShell 安装脚本
- [ ] Windows 特定的路径处理
- [ ] nvm-windows 集成

### 测试验证
- [ ] 在不同 Shell 中测试安装
- [ ] 验证自动配置功能
- [ ] 测试卸载流程
- [ ] 跨平台兼容性测试

### 阶段产出
✅ 支持多 Shell 的完整安装体验

---

## 第三阶段：性能优化
**目标**: 提升响应速度和用户体验
**预期时间**: 1-2 天
**成功标准**: 切换延迟 < 500ms，智能缓存工作正常

### 任务清单

#### 3.1 缓存系统 ⏳
- [ ] `src/core/cache.sh` - 缓存机制实现
  - .nvmrc 文件位置缓存
  - 版本信息缓存
  - TTL 过期机制
  - 缓存清理功能

#### 3.2 性能优化策略 ⏳
- [ ] 智能跳过重复处理
  - 检测目录是否真正改变
  - 避免重复的版本切换
  - 缓存命中优化

- [ ] 目录排除机制
  - 系统目录排除 (/tmp, /var 等)
  - 用户自定义排除列表
  - 性能敏感路径优化

- [ ] 延迟加载优化
  - 按需加载 NVM
  - 模块懒加载
  - 启动时间优化

#### 3.3 高级配置 ⏳
- [ ] 性能调优参数
  - 缓存 TTL 配置
  - 排除目录配置
  - 日志级别动态调整

- [ ] 更多配置选项
  - 自动安装缺失版本
  - 版本切换确认
  - 自定义提示消息

### 测试验证
- [ ] 性能基准测试
- [ ] 缓存有效性测试
- [ ] 大量目录切换测试
- [ ] 内存使用监控

### 阶段产出
✅ 高性能的生产就绪版本

---

## 第四阶段：测试和文档
**目标**: 完善测试覆盖和用户文档
**预期时间**: 2-3 天
**成功标准**: 完整的测试覆盖，用户可以轻松上手使用

### 任务清单

#### 4.1 测试套件 ⏳
- [ ] `tests/unit/` - 单元测试
  - 核心函数测试
  - 边界条件测试
  - 错误处理测试

- [ ] `tests/integration/` - 集成测试
  - Shell 集成测试
  - 端到端流程测试
  - 多场景验证

- [ ] 跨平台兼容性验证
  - macOS 测试
  - Linux 测试
  - Windows 测试

#### 4.2 完整文档 ⏳
- [ ] `docs/installation.md` - 安装指南
  - 系统要求
  - 详细安装步骤
  - 常见问题解决

- [ ] `docs/configuration.md` - 配置说明
  - 所有配置选项说明
  - 配置文件示例
  - 高级配置技巧

- [ ] `docs/troubleshooting.md` - 故障排除
  - 常见问题和解决方案
  - 调试技巧
  - 日志分析指南

- [ ] 使用示例和最佳实践
  - 典型使用场景
  - 项目集成指南
  - 性能优化建议

#### 4.3 发布准备 ⏳
- [ ] `LICENSE` 文件 (推荐 MIT)
- [ ] `CONTRIBUTING.md` - 贡献指南
- [ ] `CHANGELOG.md` - 版本变更记录
- [ ] GitHub 发布流程
- [ ] 版本标记和发布说明

### 测试验证
- [ ] 完整的回归测试
- [ ] 文档准确性验证
- [ ] 用户体验测试
- [ ] 发布流程测试

### 阶段产出
✅ 可发布到 GitHub 的完整开源项目

---

## 实施建议

### 开发环境准备
- 确保已安装 nvm 或 nvm-windows
- 准备不同版本的 Node.js 用于测试
- 设置多种 Shell 环境进行测试

### 质量控制
- 每个阶段完成后进行充分测试
- 保持代码简洁和可读性
- 添加充分的错误处理和日志

### 版本控制
- 每个阶段完成后创建 git tag
- 保持清晰的 commit 历史
- 及时更新文档

### 社区准备
- 选择合适的开源许可证
- 准备清晰的项目介绍
- 设置 GitHub 项目模板和标签