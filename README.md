<p align="center">
  <img src="screenshots/icon.png" alt="Fry TFTP Server" width="128" height="128">
</p>

<h1 align="center">Fry TFTP Server</h1>

<p align="center">
  <strong>High-performance, cross-platform TFTP server with GUI, TUI, and headless modes</strong>
</p>

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

## Overview / 概览

Fry TFTP Server is a modern TFTP server built in Rust, designed for network engineers, sysadmins, and anyone who needs fast, reliable firmware distribution and PXE boot infrastructure. It supports all major TFTP RFCs including sliding window transfers (RFC 7440) for throughput up to **500+ MB/s**.

Fry TFTP Server 是一个使用 Rust 开发的高性能 TFTP 服务器，专为网络工程师、系统管理员以及需要快速可靠地分发固件和搭建 PXE 启动环境的用户设计。支持所有主流 TFTP RFC 规范，包括滑动窗口传输（RFC 7440），吞吐量最高可达 **500+ MB/s**。

### Why Fry? / 为什么选择 Fry？

- **3 interfaces** in one binary — GUI, terminal UI, or headless daemon
  **一个二进制文件，3 种界面** — GUI图形界面、终端UI或无头守护进程
- **10x faster** than traditional TFTP servers with sliding window & mmap
  **比传统 TFTP 服务器快 10 倍**，采用滑动窗口和内存映射 I/O
- **Enterprise-ready** — ACL, rate limiting, hot-reload config, service integration
  **企业级功能** — ACL访问控制、速率限制、配置热重载、服务集成
- **Cross-platform** — runs on Windows, macOS, and Linux
  **跨平台** — 支持 Windows、macOS 和 Linux
- **Multilingual** — English, Simplified Chinese, Russian, German, Spanish, French
  **多语言** — 英语、简体中文、俄语、德语、西班牙语、法语

---

## Screenshots / 截图

### GUI Mode — 10 concurrent firmware downloads at 843 MB/s
### GUI 模式 — 10 个并发固件下载，速度 843 MB/s
![GUI Dashboard](screenshots/gui.png)

### TUI Mode — Terminal interface with real-time monitoring
### TUI 模式 — 终端界面，实时监控
![TUI Dashboard](screenshots/tui.png)

---

## Features / 功能特性

| Category | 功能类别 | Details | 详情 |
|----------|----------|---------|------|
| **Protocol** | **协议** | RFC 1350, 2347 (OACK), 2348 (Blocksize), 2349 (Timeout/Tsize), 7440 (Windowsize) | RFC 1350、2347（选项协商）、2348（块大小）、2349（超时/传输大小）、7440（窗口大小） |
| **Transfer modes** | **传输模式** | Octet (binary), Netascii (text) | Octet（二进制）、Netascii（文本） |
| **Performance** | **性能** | Sliding window (up to 64 blocks), memory-mapped I/O, buffer pooling | 滑动窗口（最多64块）、内存映射 I/O、缓冲池 |
| **Security** | **安全** | IP-based ACL (whitelist/blacklist, CIDR), per-IP rate limiting, session limits, path traversal protection, symlink policy | IP访问控制（白名单/黑名单、CIDR）、单IP速率限制、会话数限制、路径遍历保护、符号链接策略 |
| **Configuration** | **配置** | TOML config with hot-reload (file watcher + SIGHUP), environment variables, CLI overrides | TOML配置，支持热重载（文件监控+SIGHUP）、环境变量、命令行覆盖 |
| **GUI mode** | **GUI模式** | Dashboard, file browser, transfer history, log viewer, config editor, ACL manager, system tray | 仪表盘、文件浏览器、传输历史、日志查看器、配置编辑器、ACL管理器、系统托盘 |
| **TUI mode** | **TUI模式** | Full-featured terminal interface with the same capabilities | 功能完整的终端界面 |
| **Headless mode** | **无头模式** | Daemon with IPC control socket (Unix/Windows named pipe) | 守护进程，支持IPC控制套接字（Unix/Windows命名管道） |
| **Deployment** | **部署** | Docker, systemd, launchd, Windows Service | Docker、systemd、launchd、Windows服务 |
| **Internationalization** | **国际化** | 6 languages: EN, ZH, RU, DE, ES, FR (auto-detected from OS locale) | 6种语言：英、中、俄、德、西、法（自动检测系统语言） |

---

## Quick Start / 快速开始

### Download / 下载

Get the latest release from the [Releases page](https://github.com/qulisun/fry-tftp-server/releases), or build from source:

从 [Releases 页面](https://github.com/qulisun/fry-tftp-server/releases) 下载最新版本，或从源码编译：

```bash
git clone https://github.com/qulisun/fry-tftp-server.git
cd fry-tftp-server
cargo build --release
```

### Run / 运行

```bash
# GUI mode (default) / GUI 模式（默认）
./target/release/fry-tftp-server

# TUI mode / TUI 模式
./target/release/fry-tftp-server --tui

# Headless daemon / 无头守护进程
./target/release/fry-tftp-server --headless

# Custom options / 自定义选项
./target/release/fry-tftp-server --headless -p 6969 -r /srv/tftp --allow-write
```

### macOS

Download the `.dmg` from [Releases](https://github.com/qulisun/fry-tftp-server/releases), open it, and drag **Fry TFTP Server** to Applications.

从 [Releases](https://github.com/qulisun/fry-tftp-server/releases) 下载 `.dmg`，打开后将 **Fry TFTP Server** 拖入应用程序。

> **Note / 注意:** On first launch, right-click the app and select **Open** (macOS Gatekeeper). Port 69 works without sudo on macOS Ventura+.
> 首次启动时，请右键点击应用并选择 **打开**（macOS Gatekeeper）。macOS Ventura+ 上端口69无需sudo权限。

### Windows

Download `fry-tftp-server-windows-x86_64.zip` from [Releases](https://github.com/qulisun/fry-tftp-server/releases), extract, and double-click `fry-tftp-server.exe`.

从 [Releases](https://github.com/qulisun/fry-tftp-server/releases) 下载 `fry-tftp-server-windows-x86_64.zip`，解压后双击 `fry-tftp-server.exe`。

> **First launch / 首次启动:** Windows SmartScreen may show "Windows protected your PC". Click **More info** → **Run anyway**.
> Windows 智能屏幕可能显示"Windows已保护你的电脑"。点击 **更多信息** → **仍要运行**。
>
> **Alternative / 替代方法:** Right-click the `.exe` → **Properties** → check **Unblock** → **OK**.
> 右键点击 `.exe` → **属性** → 勾选 **解除锁定** → **确定**。
>
> **PowerShell:** `Unblock-File -Path .\fry-tftp-server.exe`

Port 69 requires Administrator privileges on Windows. Right-click → **Run as Administrator**, or use `-p <port>` with a port above 1024.

Windows 上端口69需要管理员权限。右键 → **以管理员身份运行**，或使用 `-p <端口>` 指定大于1024的端口。

### Linux AppImage

Download `fry-tftp-server-linux-x86_64.AppImage` from [Releases](https://github.com/qulisun/fry-tftp-server/releases), then run:

从 [Releases](https://github.com/qulisun/fry-tftp-server/releases) 下载 `fry-tftp-server-linux-x86_64.AppImage`，然后运行：

```bash
chmod +x fry-tftp-server-linux-x86_64.AppImage
./fry-tftp-server-linux-x86_64.AppImage
```

The GUI now includes Simplified Chinese. The interface language is auto-detected from the OS locale and can also be changed in the GUI configuration page.

GUI 已支持简体中文。界面语言会自动检测系统语言设置，也可以在 GUI 配置页面中手动切换。

> **Linux Desktop Integration / Linux 桌面集成:** To add to your application menu, run:
> 如需添加到应用程序菜单，请运行：
> ```bash
> mv fry-tftp-server-linux-x86_64.AppImage ~/.local/bin/fry-tftp-server
> cp deploy/appimage/fry-tftp-server.desktop ~/.local/share/applications/
> ```
> The `.desktop` file is included in the AppImage at `deploy/appimage/fry-tftp-server.desktop`.
> `.desktop` 文件已包含在 AppImage 中，位于 `deploy/appimage/fry-tftp-server.desktop`。

---

## Build Options / 编译选项

| Command | 命令 | Description | 描述 |
|---------|------|-------------|------|
| `cargo build --release` | | Full build (GUI + TUI + headless) | 完整编译（GUI + TUI + 无头） |
| `cargo build --release --no-default-features` | | Headless only (minimal, for Docker/servers) | 仅无头模式（精简，用于 Docker/服务器） |
| `cargo build --release --no-default-features --features tui` | | TUI only | 仅 TUI |
| `cargo build --release --features gui` | | GUI only | 仅 GUI |

### Linux GUI Dependencies / Linux GUI 依赖

```bash
sudo apt-get install -y libglib2.0-dev libgtk-3-dev libxdo-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

### Build AppImage / 编译 AppImage

```bash
chmod +x deploy/appimage/build-appimage.sh
./deploy/appimage/build-appimage.sh
```

---

## Configuration / 配置

Configuration is loaded from platform-specific paths automatically:

配置会自动从平台特定路径加载：

| Platform | 平台 | Path | 路径 |
|----------|------|------|------|
| **macOS** | | `~/Library/Preferences/fry-tftp-server/config.toml` | `~/Library/Preferences/fry-tftp-server/config.toml` |
| **Linux** | | `~/.config/fry-tftp-server/config.toml` | `~/.config/fry-tftp-server/config.toml` |
| **Windows** | | `%APPDATA%\fry-tftp-server\config.toml` | `%APPDATA%\fry-tftp-server\config.toml` |

Override with `-c /path/to/config.toml`. See [`config/default.toml`](config/default.toml) for all options.

使用 `-c /path/to/config.toml` 覆盖。查看 [`config/default.toml`](config/default.toml) 了解所有配置项。

### Priority (highest to lowest) / 优先级（从高到低）

```
CLI flags / CLI参数  >  Environment / 环境变量  >  Config file / 配置文件  >  Built-in defaults / 内置默认值
```

### Environment Variables / 环境变量

| Variable | 变量 | Example | 示例 |
|----------|------|---------|------|
| `TFTP_SERVER_PORT` | | `69` | 端口 |
| `TFTP_SERVER_BIND_ADDRESS` | | `0.0.0.0` | 绑定地址 |
| `TFTP_SERVER_ROOT` | | `/srv/tftp` | 根目录 |
| `TFTP_SERVER_ALLOW_WRITE` | | `true` | 允许写入 |
| `TFTP_SERVER_LOG_LEVEL` | | `info` | 日志级别 |
| `TFTP_SERVER_MAX_SESSIONS` | | `100` | 最大会话数 |
| `TFTP_SERVER_IP_VERSION` | | `dual` | IP协议版本 |

---

## Deployment / 部署

### Docker

```bash
docker build -t fry-tftp .
docker run --net=host -v /srv/tftp:/srv/tftp fry-tftp
```

> **Note / 注意:** TFTP uses ephemeral UDP ports per session. `--net=host` is required for full functionality.
> TFTP 每个会话使用临时 UDP 端口。`--net=host` 是完整功能的必要条件。

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

**GUI mode / GUI 模式** — just double-click `fry-tftp-server.exe`. No console window, no setup needed.
直接双击 `fry-tftp-server.exe`，无需控制台窗口，无需安装。

**As a Windows Service / 作为 Windows 服务**（后台运行，开机自启）:

```powershell
# Install the service / 安装服务
fry-tftp-server.exe --install-service

# Start / 启动
Start-Service FryTFTPServer

# Check status / 查看状态
Get-Service FryTFTPServer

# Stop / 停止
Stop-Service FryTFTPServer

# Uninstall / 卸载
fry-tftp-server.exe --uninstall-service
```

The service runs in headless mode and can be managed via `services.msc` or PowerShell.
服务以无头模式运行，可通过 `services.msc` 或 PowerShell 管理。

---

## CLI Reference / 命令行参考

```
Usage: fry-tftp-server [OPTIONS]
用法: fry-tftp-server [选项]

Options / 选项:
      --gui                Run in GUI mode (default) / GUI 模式运行（默认）
      --tui                Run in TUI mode / TUI 模式运行
      --headless           Run in headless mode (daemon) / 无头模式运行（守护进程）
  -c, --config <FILE>      Path to config file / 配置文件路径
  -r, --root <DIR>         Root directory / 根目录
  -p, --port <PORT>        Port number (default: 69) / 端口号（默认：69）
  -b, --bind <ADDR>        Bind address (default: ::) / 绑定地址（默认：::）
      --allow-write        Allow write requests (WRQ) / 允许写请求（WRQ）
      --max-sessions <N>   Maximum parallel sessions / 最大并发会话数
      --blksize <N>        Maximum block size / 最大块大小
      --windowsize <N>     Maximum window size / 最大窗口大小
      --ip-version <V>     IP version: dual | v4 | v6 / IP协议版本
  -v, --verbose            Increase verbosity (-v, -vv, -vvv) / 增加详细程度
  -q, --quiet              Quiet mode (errors only) / 安静模式（仅错误）
  -h, --help               Print help / 打印帮助
  -V, --version            Print version / 打印版本
```

---

## RFC Compliance / RFC 规范符合性

| RFC | Title / 标题 | Status / 状态 |
|-----|--------------|:------------:|
| [RFC 1350](https://datatracker.ietf.org/doc/html/rfc1350) | TFTP Protocol (Revision 2) / TFTP 协议（第二版） | ✅ Fully implemented / 完全实现 |
| [RFC 2347](https://datatracker.ietf.org/doc/html/rfc2347) | Option Extension (OACK) / 选项扩展（OACK） | ✅ Fully implemented / 完全实现 |
| [RFC 2348](https://datatracker.ietf.org/doc/html/rfc2348) | Blocksize Option / 块大小选项 | ✅ Fully implemented / 完全实现 |
| [RFC 2349](https://datatracker.ietf.org/doc/html/rfc2349) | Timeout & Transfer Size / 超时和传输大小 | ✅ Fully implemented / 完全实现 |
| [RFC 7440](https://datatracker.ietf.org/doc/html/rfc7440) | Windowsize Option / 窗口大小选项 | ✅ Fully implemented / 完全实现 |

---

## Performance / 性能

Benchmarked on Apple Silicon (M-series), localhost, single session:

在 Apple Silicon（M系列）上本地单会话测试：

| File Size / 文件大小 | Default (512B/win=1) / 默认（512字节/窗口=1） | Optimized (8KB/win=8) / 优化（8KB/窗口=8） | Max (64KB/win=16) / 最大（64KB/窗口=16） |
|:---------:|:--------------------:|:---------------------:|:-----------------:|
| 1 MB | 19 MB/s | 993 MB/s | 2,365 MB/s |
| 10 MB | 31 MB/s | 1,227 MB/s | 3,950 MB/s |
| 100 MB | 30 MB/s | 1,031 MB/s | 4,009 MB/s |
| 1 GB | -- | 212 MB/s | 3,392 MB/s |
| 5 GB | -- | 235 MB/s | 207 MB/s |

---

## TUI Keybindings / TUI 快捷键

| Key / 按键 | Action / 操作 |
|-----|---------|
| `1`-`7` | Switch tabs / 切换标签页 |
| `Tab` / `Shift+Tab` | Next / previous tab / 下一个/上一个标签页 |
| `j` / `k` | Scroll down / up / 下/上滚动 |
| `Enter` | Select / open / edit / 选择/打开/编辑 |
| `Esc` | Back / cancel / clear filter / 返回/取消/清除筛选 |
| `/` | Search / filter / 搜索/筛选 |
| `s` | Start / stop server / 启动/停止服务器 |
| `r` | Reload config & files / 重载配置和文件 |
| `a` / `e` / `d` | Add / edit / delete ACL rule / 添加/编辑/删除 ACL 规则 |
| `q` | Quit / 退出 |
| `?` | Help overlay / 帮助 |

---

## Firewall / 防火墙

TFTP requires UDP port 69 (main) plus ephemeral ports for sessions.

TFTP 需要 UDP 端口 69（主端口）及会话用临时端口。

```bash
# Linux (ufw) / Linux（ufw）
sudo ufw allow 69/udp

# macOS — port 69 works without sudo on Ventura+
# macOS — Ventura+ 上端口69无需sudo

# Windows (PowerShell, run as Admin) / Windows（PowerShell，以管理员身份）
New-NetFirewallRule -DisplayName "TFTP" -Direction Inbound -Protocol UDP -LocalPort 69 -Action Allow
```

---

## Contributing / 贡献

Contributions are welcome! Please:

欢迎贡献！请按以下步骤：

1. Fork the repository / Fork 本仓库
2. Create a feature branch (`git checkout -b feature/my-feature`) / 创建功能分支
3. Run `cargo fmt` and `cargo clippy --all-features` before committing / 提交前运行 `cargo fmt` 和 `cargo clippy --all-features`
4. Ensure all tests pass (`cargo test`) / 确保所有测试通过（`cargo test`）
5. Submit a Pull Request / 提交 Pull Request

---

## License / 许可证

This project is licensed under the [MIT License](LICENSE).

本项目采用 [MIT 许可证](LICENSE)。

---

<p align="center">
  <sub>Built with Rust, egui, ratatui, tokio</sub><br>
  <sub>Created by <a href="https://github.com/qulisun">Viacheslav Gordeev</a></sub>
</p>
