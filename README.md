# Autoscript2


[![DOI](https://img.shields.io/badge/DOI-10.6084/m9.figshare.32172564-blue.svg)](https://doi.org/10.6084/m9.figshare.32172564)

**Autoscript2** is a robust, recursive file manager developed in Rust. It is designed to be "crash-proof" through deep recursive search and robust error handling.

---

## 🚀 Key Features

* **Recursive Discovery**: Implements deep recursive search to find and manage files throughout directory structures.
* **Configurable Ignore Lists**: Supports an optional text file containing paths to ignore, passed as a command-line argument.
* **Safety-First Design**: Specifically refined to be fully functional and resilient during complex file operations.
* **Stateless Execution**: Operates directly on your file system without creating unnecessary metadata.

---

## ⚠️ Important: Editor Requirement

> [!IMPORTANT]
> **Autoscript2 currently supports the `nano` text editor only.**
> The application specifically calls `nano` to handle file editing during the execution loop. Ensure `nano` is installed and available in your system's `PATH`.

---

## 🛠️ Installation

Ensure you have the Rust toolchain installed. Clone the repository from Codeberg and build using Cargo:

```bash
git clone [https://codeberg.org/comdotharman/Autoscript2](https://codeberg.org/comdotharman/Autoscript2)
cd Autoscript2
cargo build --release

