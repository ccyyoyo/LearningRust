# Cargo 使用指南

Cargo 是 Rust 的官方建置系統和套件管理器。它負責處理許多任務，讓開發流程更順暢，例如：

*   建立新專案
*   下載並編譯專案的依賴項 (稱為 "crates")
*   建置與運行專案

## 1. 創建新專案

您可以使用 `cargo new` 指令來建立一個新的 Rust 專案。

```bash
# 建立一個名為 "hello_world" 的二進位 (executable) 應用程式
cargo new hello_world

# 建立一個名為 "my_library" 的函式庫 (library)
cargo new my_library --lib

# 不建立 .git/
# 不建立 .gitignore
# 將專案視為普通資料夾
cargo new hello-rust --vcs none
```

這個指令會建立一個新的目錄，其中包含：

*   `Cargo.toml`: 專案的設定檔。
*   `src/main.rs` (若是應用程式) 或 `src/lib.rs` (若是函式庫): 程式碼的進入點。
*   `.gitignore`: 一個預設的 Git 忽略清單。

## 2. 主要 Cargo 指令

在您的專案目錄中，您可以使用以下常用指令：

*   `cargo build`: 編譯您的專案。
    *   預設為偵錯 (debug) 模式。
    *   執行檔會放在 `target/debug/` 目錄下。
    *   若要進行最佳化以供發布，請使用 `cargo build --release`，執行檔會放在 `target/release/`。

*   `cargo run`: 編譯並執行您的專案。
    *   這是一個方便的捷徑，相當於先執行 `cargo build` 再手動執行產生的執行檔。

*   `cargo check`: 快速檢查您的程式碼。
    *   這個指令會檢查程式碼以確保它可以成功編譯，但不會實際產生執行檔。
    *   由於它跳過了產生執行檔的步驟，所以速度比 `cargo build` 快很多。在開發過程中，這對於快速檢查語法錯誤非常有用。

## 3. Cargo.toml (設定檔)

`Cargo.toml` 是專案的設定檔，使用 TOML 格式。它包含了所有關於您專案的元數據和依賴項。

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# 在這裡加入您專案的依賴項
# 例如: rand = "0.8.5"
# hammer = { version = "0.5.0"}
# color = { git = "https://github.com/bjz/color-rs" }
# geometry = { path = "crates/geometry" }

```

*   **`[package]`**: 包含專案的基本資訊，例如名稱、版本和使用的 Rust 版本 (`edition`)。
*   **`[dependencies]`**: 這是您宣告專案所需外部函式庫 (crates) 的地方。當您加入一個依賴項並執行 `cargo build` 時，Cargo 會自動從官方套件倉庫 [crates.io](https://crates.io/) 下載並編譯它。

## 4. Cargo.lock (鎖定依賴版本)

`Cargo.lock` 檔案是 Cargo 自動產生的，**您不應該手動編輯它**。

*   **用途**: 確保建置的可重現性 (Reproducible Builds)。
*   **運作方式**: 當您第一次建置專案時，Cargo 會計算出所有符合 `Cargo.toml` 中條件的依賴項的確切版本，並將這些版本號碼記錄在 `Cargo.lock` 檔案中。
*   **好處**: 當您或其他開發者未來再次建置這個專案時，Cargo 會讀取 `Cargo.lock` 並下載完全相同的依賴版本。這可以避免因為依賴項更新而導致的非預期錯誤，解決了「但在我的電腦上可以跑啊！」的問題。

當您想要更新依賴項版本時，可以使用 `cargo update` 指令。Cargo 會在 `Cargo.toml` 允許的範圍內尋找最新的版本，並更新 `Cargo.lock` 檔案。

## 5. 標準的套件目錄結構 (詳細說明)

一個標準的 Cargo 套件目錄結構有助於組織您的程式碼。Cargo 會根據檔案位置來識別它們的用途，這種「約定優於配置」的模式讓專案管理更為簡潔。

### 套件的根檔案：`src/lib.rs` 與 `src/main.rs`

一個套件 (Package) 可以包含一個函式庫 crate，和任意數量的二進位 crate。

*   **`src/lib.rs`**: 這是 **函式庫 crate (library crate)** 的根檔案。如果您的專案是一個函式庫，旨在被其他專案引用，那麼您的主要程式碼會從這個檔案開始組織。一個套件最多只能有一個函式庫。

*   **`src/main.rs`**: 這是 **預設二進位 crate (binary crate)** 的根檔案。如果您的專案是一個可執行的應用程式，那麼 `main` 函數（程式的進入點）就應該放在這個檔案裡。

一個套件可以同時擁有 `src/lib.rs` 和 `src/main.rs`。這種情況很常見，表示專案本身是一個可執行的應用程式，但它也將其核心邏輯封裝成一個函式庫，供 `main.rs` 或其他外部專案使用。

### 多個執行檔：`src/bin/*.rs`

如果您的專案需要產生多個執行檔（例如，一個主程式，一個管理工具），您可以將它們放在 `src/bin/` 目錄下。

*   **結構**: `src/bin/` 目錄下的每一個 `.rs` 檔案都會被編譯成一個獨立的執行檔。
*   **範例**: 如果您有 `src/bin/server.rs` 和 `src/bin/cli.rs`，Cargo 會產生 `server` 和 `cli` 兩個執行檔。
*   **執行**: 您可以使用 `cargo run --bin <執行檔名稱>` 來單獨執行某個檔案，例如 `cargo run --bin server`。

### 測試：`tests/` 目錄

Rust 的測試分為兩類：單元測試和整合測試。

*   **單元測試 (Unit Tests)**: 通常放在與被測試程式碼相同的 `src` 檔案中，用於測試模組內部的私有功能。
*   **整合測試 (Integration Tests)**: 放在專案根目錄的 `tests/` 目錄下。
    *   **用途**: 這些測試從外部使用者的角度來測試您的函式庫 crate 的公開 API。
    *   **運作**: `tests/` 目錄下的每個 `.rs` 檔案都會被編譯成一個獨立的測試 crate。它們會像外部專案一樣連結到您的函式庫。
    *   **限制**: 整合測試只能測試函式庫的功能 (`src/lib.rs`)，無法直接測試二進位執行檔 (`src/main.rs`)。
    *   **執行**: 使用 `cargo test` 會同時執行所有單元測試和整合測試。

### 範例程式：`examples/` 目錄

這個目錄存放了如何使用您的函式庫的範例程式。

*   **用途**: 為您的函式庫使用者提供清晰的使用範例。
*   **結構**: `examples/` 目錄下的每個 `.rs` 檔案都是一個獨立的小程式。
*   **執行**: 您可以使用 `cargo run --example <範例名稱>` 來執行特定的範例，例如 `cargo run --example simple_usage`。

### 效能測試：`benches/` 目錄

這個目錄用於存放效能基準測試 (benchmarks)，用來衡量您程式碼的執行速度和記憶體使用情況。

*   **用途**: 識別效能瓶頸並在修改後驗證效能是否提升。
*   **執行**: 使用 `cargo bench` 來執行所有基準測試。
