# [老高之家] AES 加解密小工具

> 一款纯本地、无广告的 AES 文件加密/解密工具，由 `Boss_Gao@qq.com` 开发维护。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Platform](https://img.shields.io/badge/platform-Windows-blue)
![Version](https://img.shields.io/badge/version-1.3.0-brightgreen)

---

## ✨ 功能亮点

### 核心加解密

- 🔒 基于 **AES-256-CBC** 算法，安全可靠
- 📁 支持任意文件加密，输出 `.aes` 格式文件
- 🔓 解密 `.aes` 文件，恢复原始数据
- 📂 自定义输出目录，或直接输出到程序所在目录
- 🧹 加密后可选择自动删除源文件（带安全提示）

### 文件名校验与保护

- 📏 **输出文件名长度校验**：超过 250 字符时阻止加/解密并提示用户
- 🔍 **同名文件检测**：选择文件后自动检查输出目录，若存在同名文件则以红色文字（`#c71d23`）警告"此目录已有相同文件，请重命名"
- 🔢 **自动重命名机制**：即使存在同名文件仍执行操作时，自动在文件名最后一个后缀前添加 `(n)` 序号（从 `(2)` 开始递增），例如 `test.abc.txt.aes` → `test.abc.txt(2).aes`，避免文件覆盖
- ✏️ 用户主动修改输出名称后自动触发重复检测，无同名文件时隐藏警告

### 密码安全

- 🛡️ 密码内存安全处理（`Zeroize`），防止泄露
- 🟢 **密码强度实时评估**：加密时以红/黄/绿三色进度条 + 文字标签展示"极弱 / 弱 / 中等 / 强 / 极强"等级
- 🔠 **Caps Lock 检测**：大写锁定开启时显示橙色提醒

### 交互体验

- 🎨 纯 GUI 界面（基于 `egui`），无 CMD 窗口，窗口可自由拉伸
- 📊 **自定义圆角渐变进度条**：加/解密操作可视化，动画至少持续 1.5 秒
- 🚫 防多开、关闭前警告、30 秒超时取消等贴心交互
- 🖱️ 右下角版权链接（含 GitHub / Gitee 仓库入口），鼠标悬停显示手型光标
- 🌐 界面文本全部使用 `obfstr!` 混淆，提升安全性
- ⚖️ 完全免费，源码未开放但二进制可自由使用（MIT 许可证）

---

## 📥 下载与使用

1. 前往 [Releases](https://github.com/himtdl/laogao-software-aescrypt/releases) 页面下载最新版 `.exe` 文件
2. 双击运行，无需安装
3. 切换 **解密 / 加密** 标签页，选择文件
4. 输入密码，点击开始
5. 操作成功后，文件将保存到指定目录（默认程序所在目录）

> **注意**：本工具不会联网，无后门，所有加解密操作均在本地完成。

---

## 🔐 加密技术细节

- **加密算法**：AES-256-CBC
- **密钥派生**：PBKDF2-HMAC-SHA256（默认迭代次数 8192）
- **文件格式**：兼容 [AESCrypt](https://www.aescrypt.com/) 格式，可与其他实现互通
- **密码保护**：内存中使用 `Zeroizing<String>` 字符串，Drop 时自动清零
- **密码强度**：基于 [zxcvbn](https://github.com/dropbox/zxcvbn) 算法实时评估

---

## 📜 许可与免责

本项目采用 **MIT 许可证**，详见 [LICENSE](LICENSE) 文件。

**您被允许：**

- 在任何场景下免费使用、复制、修改、分发本软件
- 用于商业或非商业目的

**但请注意：**

- 本软件 **按"原样"提供**，不提供任何明示或暗示的担保
- 您须自行承担使用本软件可能带来的数据丢失、损坏等风险
- **严禁用于任何非法用途**（如破解他人文件、违法加密勒索等）

---

## 📦 第三方依赖与许可

本工具使用了以下优秀的开源库，感谢所有贡献者！

| 库 | 许可证 |
|---|---|
| [aescrypt_rs](https://crates.io/crates/aescrypt_rs) | MIT / Apache-2.0 |
| [egui / eframe](https://github.com/emilk/egui) | MIT / Apache-2.0 |
| [rfd](https://github.com/PolyMeilex/rfd) | MIT / Apache-2.0 |
| [single_instance](https://crates.io/crates/single_instance) | MIT / Apache-2.0 |
| [image](https://github.com/image-rs/image) | MIT / Apache-2.0 |
| [zeroize](https://crates.io/crates/zeroize) | MIT / Apache-2.0 |
| [obfstr](https://crates.io/crates/obfstr) | MIT / Apache-2.0 |
| [zxcvbn](https://crates.io/crates/zxcvbn) | MIT / Apache-2.0 |
| [open](https://crates.io/crates/open) | MIT / Apache-2.0 |
| [winapi](https://crates.io/crates/winapi) | MIT / Apache-2.0 |

上述库的完整许可证文本已随二进制分发附带（若需要，可查看 Cargo.lock 或上游仓库）。

---

## 📝 版本历史

详见 [CHANGELOG.md](CHANGELOG.md)


## 👤 作者与联系

- 作者：Boss_Gao@qq.com
- GitHub：[https://github.com/himtdl/laogao-software-aescrypt](https://github.com/himtdl/laogao-software-aescrypt)
- Gitee（国内同步）：[https://gitee.com/himtdl/laogao-software-aescrypt](https://gitee.com/himtdl/laogao-software-aescrypt)
- 反馈问题：请在 GitHub 仓库提交 [Issue](https://github.com/himtdl/laogao-software-aescrypt/issues)

---

**如果喜欢这个工具，欢迎给个 ⭐ Star 支持！**
