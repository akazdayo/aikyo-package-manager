# APM (Aikyo Package Manager)

[Aikyo](https://github.com/marukun712/aikyo)のAI Companion用のRust製パッケージマネージャーです。

## 概要

APMは、Gitリポジトリを`aikyo_dependencies/`ディレクトリに自動的にクローンし、`apm.toml`設定ファイルを通じて管理するツールです。プラグインシステムの構築と管理を簡単にします。

## インストール

```bash
brew install akazdayo/tap/apm
```

## 使用方法

### 初期化

```bash
apm init
```

### プラグインの追加

```bash
apm add <GitリポジトリのURL>
```

例：
```bash
apm add https://github.com/user/plugin-name.git
```

### プラグインの同期

設定されたプラグインを`tools/`ディレクトリに同期（クローン）します：

```bash
apm sync
```

### その他のコマンド

- `apm remove` - プラグインの削除（実装予定）

## 設定ファイル

`apm.toml`ファイルでプラグインを管理します：

```toml
[project]
plugins = [
    "https://github.com/user/plugin1.git",
    "https://github.com/user/plugin2.git"
]
tools_dir = "./aikyo_dependencies"
```

## プロジェクト構成
- `apm.toml` - プロジェクト設定ファイル
- `aikyo_dependencies/` - プラグインが配置されるディレクトリ

## 開発

### ビルド

```bash
cargo build
```

### テスト実行

```bash
cargo test
```

### 型チェック

```bash
cargo check
```

## トラブルシューティング
### Syncしてもクローンされない
- apm.tomlに含まれているURLの末尾に`.git`が含まれていない
