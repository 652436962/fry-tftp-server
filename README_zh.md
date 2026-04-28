<p align="center">
  <img src="screenshots/icon.png" alt="Fry TFTP Server" width="128" height="128">
</p>

<h1 align="center">Fry TFTP Server</h1>

<p align="center">
  <strong>高性能、跨平台 TFTP 服务器，支持 GUI、TUI 和无头模式</strong>
</p>

<p align="center">
  <a href="https://github.com/qulisun/fry-tftp-server/actions"><img src="https://github.com/qulisun/fry-tftp-server/workflows/CI/badge.svg" alt="CI"></a>
  <a href="https://github.com/qulisun/fry-tftp-server/releases"><img src="https://img.shields.io/github/v/release/qulisun/fry-tftp-server?color=blue" alt="Release"></a>
  <img src="https://img.shields.io/badge/rust-1.75%2B-orange" alt="Rust 1.75+">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey" alt="Platform">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-green" alt="License: MIT"></a>
  <img src="https://img.shields.io/badge/i18n-EN%20%7C%20ZH%20%7C%20RU%20%7C%20DE%20%7C%20ES%20%7C%20FR-blueviolet" alt="Languages">
</p>

---

## 概览

Fry TFTP Server 是一个使用 Rust 开发的高性能 TFTP 服务器，专为网络工程师、系统管理员以及需要快速可靠地分发固件和搭建 PXE 启动环境的用户设计。支持所有主流 TFTP RFC 规范，包括滑动窗口传输（RFC 7440），吞吐量最高可达 **500+ MB/s**。

### 为什么选择 Fry？

- **一个二进制文件，3 种界面** — GUI 图形界面、终端 UI 或无头守护进程
- **比传统 TFTP 服务器快 10 倍**，采用滑动窗口和内存映射 I/O
- **企业级功能** — ACL 访问控制、速率限制、配置热重载、服务集成
- **跨平台** — 支持 Windows、macOS 和 Linux
- **多语言** — 英语、简体中文、俄语、德语、西班牙语、法语

---

## 截图

### GUI 模式 — 10 个并发固件下载，速度 843 MB/s
![GUI Dashboard](screenshots/gui.png)

### TUI 模式 — 终端界面，实时监控
![TUI Dashboard](screenshots/tui.png)

---

## 功能特性

| 类别 | 详情 |
|------|------|
| **协议** | RFC 1350、2347（选项协商 OACK）、2348（块大小）、2349（超时/传输大小）、7440（窗口大小） |
| **传输模式** | Octet（二进制）、Netascii（文本） |
| **性能** | 滑动窗口（最多64块）、内存映射 I/O、缓冲池 |
| **安全** | IP 访问控制（白名单/黑名单、CIDR）、单 IP 速率限制、会话数限制、路径遍历保护、符号链接策略 |
| **配置** | TOML 配置，支持热重载（文件监控 + SIGHUP）、环境变量、命令行覆盖 |
| **GUI 模式** | 仪表盘、文件浏览器、传输历史、日志查看器、配置编辑器、ACL 管理器、系统托盘 |
| **TUI 模式** | 功能完整的终端界面 |
| **无头模式** | 守护进程，支持 IPC 控制套接字（Unix/Windows 命名管道） |
| **部署** | Docker、systemd、launchd、Windows 服务 |
| **国际化** | 6 种语言：英、中、俄、德、西、法（自动检测系统语言） |

---

## 快速开始

### 下载

从 [Releases 页面](https://github.com/qulisun/fry-tftp-server/releases) 下载最新版本，或从源码编译：

```bash
git clone https://github.com/qulisun/fry-tftp-server.git
cd fry-tftp-server
cargo build --release
```

### 运行

```bash
# GUI 模式（默认）
./target/release/fry-tftp-server

# TUI 模式
./target/release/fry-tftp-server --tui

# 无头守护进程
./target/release/fry-tftp-server --headless

# 自定义选项
./target/release/fry-tftp-server --headless -p 6969 -r /srv/tftp --allow-write
```

### macOS

从 [Releases](https://github.com/qulisun/fry-tftp-server/releases) 下载 `.dmg`，打开后将 **Fry TFTP Server** 拖入应用程序。

> **注意:** 首次启动时，请右键点击应用并选择 **打开**（macOS Gatekeeper）。macOS Ventura+ 上端口 69 无需 sudo 权限。

### Windows

从 [Releases](https://github.com/qulisun/fry-tftp-server/releases) 下载 `fry-tftp-server-windows-x86_64.zip`，解压后双击 `fry-tftp-server.exe`。

> **首次启动:** Windows 智能屏幕可能显示"Windows 已保护你的电脑"。点击 **更多信息** → **仍要运行**。
>
> **替代方法:** 右键点击 `.exe` → **属性** → 勾选 **解除锁定** → **确定**。
>
> **PowerShell:** `Unblock-File -Path .\fry-tftp-server.exe`

Windows 上端口 69 需要管理员权限。右键 → **以管理员身份运行**，或使用 `-p <端口>` 指定大于 1024 的端口。

### Linux AppImage

从 [Releases](https://github.com/qulisun/fry-tftp-server/releases) 下载 `fry-tftp-server-linux-x86_64.AppImage`，然后运行：

```bash
chmod +x fry-tftp-server-linux-x86_64.AppImage
./fry-tftp-server-linux-x86_64.AppImage
```

GUI 已支持简体中文。界面语言会自动检测系统语言设置，也可以在 GUI 配置页面中手动切换。

> **Linux 桌面集成:** 如需添加到应用程序菜单，请运行：
> ```bash
> mv fry-tftp-server-linux-x86_64.AppImage ~/.local/bin/fry-tftp-server
> cp deploy/appimage/fry-tftp-server.desktop ~/.local/share/applications/
> ```
> `.desktop` 文件已包含在 AppImage 中，位于 `deploy/appimage/fry-tftp-server.desktop`。

---

## 编译选项

| 命令 | 描述 |
|------|------|
| `cargo build --release` | 完整编译（GUI + TUI + 无头） |
| `cargo build --release --no-default-features` | 仅无头模式（精简，用于 Docker/服务器） |
| `cargo build --release --no-default-features --features tui` | 仅 TUI |
| `cargo build --release --features gui` | 仅 GUI |

### Linux GUI 依赖

```bash
sudo apt-get install -y libglib2.0-dev libgtk-3-dev libxdo-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

### 编译 AppImage

```bash
chmod +x deploy/appimage/build-appimage.sh
./deploy/appimage/build-appimage.sh
```

---

## 配置

配置会自动从平台特定路径加载：

| 平台 | 路径 |
|------|------|
| **macOS** | `~/Library/Preferences/fry-tftp-server/config.toml` |
| **Linux** | `~/.config/fry-tftp-server/config.toml` |
| **Windows** | `%APPDATA%\fry-tftp-server\config.toml` |

使用 `-c /path/to/config.toml` 覆盖。查看 [`config/default.toml`](config/default.toml) 了解所有配置项。

### 优先级（从高到低）

```
命令行参数  >  环境变量（TFTP_SERVER_*）  >  配置文件  >  内置默认值
```

### 环境变量

| 变量 | 示例 | 说明 |
|------|------|------|
| `TFTP_SERVER_PORT` | `69` | 端口号 |
| `TFTP_SERVER_BIND_ADDRESS` | `0.0.0.0` | 绑定地址 |
| `TFTP_SERVER_ROOT` | `/srv/tftp` | 根目录 |
| `TFTP_SERVER_ALLOW_WRITE` | `true` | 允许写入 |
| `TFTP_SERVER_LOG_LEVEL` | `info` | 日志级别 |
| `TFTP_SERVER_MAX_SESSIONS` | `100` | 最大会话数 |
| `TFTP_SERVER_IP_VERSION` | `dual` | IP 协议版本 |

---

## 部署

### Docker

```bash
docker build -t fry-tftp .
docker run --net=host -v /srv/tftp:/srv/tftp fry-tftp
```

> **注意:** TFTP 每个会话使用临时 UDP 端口。`--net=host` 是完整功能的必要条件。

### systemd (Linux)

```bash
sudo cp deploy/fry-tftp-server.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now fry-tftp-server
```

### launchd (macOS)

```bash
sudo cp deploy/com.fry-tftp-server.plist /Library/LaunchDaemons/
sudo launchctl load /Library/LaunchDaemons/com.fry-tftp-server.plist
```

### Windows

**GUI 模式** — 直接双击 `fry-tftp-server.exe`，无需控制台窗口，无需安装。

**作为 Windows 服务**（后台运行，开机自启）:

```powershell
# 安装服务
fry-tftp-server.exe --install-service

# 启动
Start-Service FryTFTPServer

# 查看状态
Get-Service FryTFTPServer

# 停止
Stop-Service FryTFTPServer

# 卸载
fry-tftp-server.exe --uninstall-service
```

服务以无头模式运行，可通过 `services.msc` 或 PowerShell 管理。

---

## 命令行参考

```
用法: fry-tftp-server [选项]

选项:
      --gui                GUI 模式运行（默认）
      --tui                TUI 模式运行
      --headless           无头模式运行（守护进程）
  -c, --config <文件>      配置文件路径
  -r, --root <目录>        根目录
  -p, --port <端口>        端口号（默认：69）
  -b, --bind <地址>        绑定地址（默认：::）
      --allow-write        允许写请求（WRQ）
      --max-sessions <N>   最大并发会话数
      --blksize <N>        最大块大小
      --windowsize <N>     最大窗口大小
      --ip-version <V>     IP 协议版本：dual | v4 | v6
  -v, --verbose            增加详细程度（-v, -vv, -vvv）
  -q, --quiet              安静模式（仅错误）
  -h, --help               打印帮助
  -V, --version            打印版本
```

---

## RFC 规范符合性

| RFC | 标题 | 状态 |
|-----|------|:----:|
| [RFC 1350](https://datatracker.ietf.org/doc/html/rfc1350) | TFTP 协议（第二版） | ✅ 完全实现 |
| [RFC 2347](https://datatracker.ietf.org/doc/html/rfc2347) | 选项扩展（OACK） | ✅ 完全实现 |
| [RFC 2348](https://datatracker.ietf.org/doc/html/rfc2348) | 块大小选项 | ✅ 完全实现 |
| [RFC 2349](https://datatracker.ietf.org/doc/html/rfc2349) | 超时和传输大小 | ✅ 完全实现 |
| [RFC 7440](https://datatracker.ietf.org/doc/html/rfc7440) | 窗口大小选项 | ✅ 完全实现 |

---

## 性能

在 Apple Silicon（M 系列）上本地单会话测试：

| 文件大小 | 默认（512字节/窗口=1） | 优化（8KB/窗口=8） | 最大（64KB/窗口=16） |
|:--------:|:----------------------:|:------------------:|:--------------------:|
| 1 MB | 19 MB/s | 993 MB/s | 2,365 MB/s |
| 10 MB | 31 MB/s | 1,227 MB/s | 3,950 MB/s |
| 100 MB | 30 MB/s | 1,031 MB/s | 4,009 MB/s |
| 1 GB | -- | 212 MB/s | 3,392 MB/s |
| 5 GB | -- | 235 MB/s | 207 MB/s |

---

## TUI 快捷键

| 按键 | 操作 |
|------|------|
| `1`-`7` | 切换标签页 |
| `Tab` / `Shift+Tab` | 下一个/上一个标签页 |
| `j` / `k` | 下/上滚动 |
| `Enter` | 选择/打开/编辑 |
| `Esc` | 返回/取消/清除筛选 |
| `/` | 搜索/筛选 |
| `s` | 启动/停止服务器 |
| `r` | 重载配置和文件 |
| `a` / `e` / `d` | 添加/编辑/删除 ACL 规则 |
| `q` | 退出 |
| `?` | 帮助 |

---

## 防火墙

TFTP 需要 UDP 端口 69（主端口）及会话用临时端口。

```bash
# Linux（ufw）
sudo ufw allow 69/udp

# macOS — Ventura+ 上端口 69 无需 sudo

# Windows（PowerShell，以管理员身份）
New-NetFirewallRule -DisplayName "TFTP" -Direction Inbound -Protocol UDP -LocalPort 69 -Action Allow
```

---

## 贡献

欢迎贡献！请按以下步骤：

1. Fork 本仓库
2. 创建功能分支（`git checkout -b feature/my-feature`）
3. 提交前运行 `cargo fmt` 和 `cargo clippy --all-features`
4. 确保所有测试通过（`cargo test`）
5. 提交 Pull Request

---

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

---

<p align="center">
  <sub>Built with Rust, egui, ratatui, tokio</sub><br>
  <sub>Created by <a href="https://github.com/qulisun">Viacheslav Gordeev</a></sub>
</p>
