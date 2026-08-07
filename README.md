# 🦀 MemVault - High-Performance Memory Token Hunter

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)]()

**MemVault** is a blazing-fast, cross-platform memory analysis tool written in **Rust**. It is designed for cybersecurity researchers and penetration testers to extract sensitive information (such as **JWT Tokens, API Keys, Discord Tokens, and Bearer Tokens**) from running process memory.

---

## ✨ Features

- **⚡ Blazing Fast**: Uses SIMD-accelerated pattern matching (`memchr`) to scan gigabytes of RAM in seconds.
- **🧠 Smart Filtering**: Integrates **Shannon Entropy** calculation to filter out common words and only return high-randomness strings (tokens).
- **🖥️ Cross-Platform**: Works seamlessly on **Windows**, **Linux**, and **macOS**.
- **🛠️ Dual Utility**:
  - **Scan**: Scan a live process for specific byte patterns.
  - **Dump**: Dump the entire memory of a process to a raw binary file for offline analysis.
- **🎨 Rich CLI**: Beautiful colored output with real-time progress bars.

---

## 📦 Installation

### Prerequisites
- Rust (Install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/memvault.git
cd memvault

# Build in release mode (optimized)
cargo build --release
```

The binary will be located at `target/release/memvault` (or `memvault.exe` on Windows).

---

## 🚀 Usage

**⚠️ Important**: You must run this tool with **Administrator/root privileges** to access other process memory.

- **Windows**: Run PowerShell/CMD as **Administrator**.
- **Linux/macOS**: Use `sudo`.

### Commands

#### 1. Scan a Process for a Pattern

Scans the memory of a target process (PID) for a specific byte pattern (e.g., `eyJ` for JWT tokens).

```bash
# Basic scan for JWT tokens
./memvault scan --pid 1234 --pattern "eyJ"

# Scan for GitHub tokens with custom entropy threshold
./memvault scan --pid 1234 --pattern "ghp_" --entropy 6.0

# Scan for Bearer tokens (lower entropy threshold to catch structured text)
./memvault scan --pid 1234 --pattern "Bearer" --entropy 4.0
```

#### 2. Dump Process Memory

Dumps the entire readable memory of a process to a binary file.

```bash
./memvault dump --pid 1234 --output "C:\dumps\process.dmp"
```

#### 3. Help

```bash
./memvault --help
```

---

## 🔍 How It Works

1.  **Process Attachment**: Attaches to the target process using `procmod-core`.
2.  **Region Enumeration**: Lists all readable memory regions.
3.  **Pattern Scanning**: Uses `memchr` (SIMD) to rapidly find the specified byte pattern.
4.  **Entropy Analysis**: Calculates Shannon Entropy on the surrounding bytes. If the entropy is above the threshold (default `5.8`), it is flagged as a potential token.
5.  **Output**: Displays the extracted tokens in a clean, color-coded format.

---

## 🧪 Example Output

```bash
$ ./memvault scan --pid 4567 --pattern "eyJ"

[*] Scanning Process PID: 4567 for pattern 'eyJ' (Entropy Threshold: 5.8)
[+] Found 1 potential tokens:
  > eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
```

---

## 🛡️ Legal Disclaimer

> **This tool is intended for educational purposes and authorized security testing only.**
>
> The user assumes all responsibility for compliance with local laws and regulations. Unauthorized access to computer systems is illegal. The author is not liable for any misuse or damage caused by this software.

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/Jayasankha-dev).

---

## 📚 Acknowledgements

- [procmod-core](https://crates.io/crates/procmod-core) - Cross-platform process interaction.
- [memchr](https://crates.io/crates/memchr) - SIMD-accelerated search.
- [clap](https://crates.io/crates/clap) - Command line argument parsing.
