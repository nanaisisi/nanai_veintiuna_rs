# Nanai Veintiuna RS - ブラックジャックゲーム

## 概要

**Nanai Veintiuna RS** は、Rustプログラミング言語で実装された完全機能のブラックジャックゲームです。伝統的なブラックジャックルールに加え、ダブルダウン、スプリット、サレンダーなどの高度な機能を備えています。

## 主な特徴

### 🎯 ゲーム機能

- **基本ルール**: 標準的なブラックジャックルール（21に近づける）
- **高度なルール**:
  - ダブルダウン（ベット額2倍、1枚のみ引く）
  - スプリット（同じ値の手札を分割）
  - サレンダー（降参、ベット額半額返却）
- **ブラックジャック判定**: 最初の2枚で21の特別処理
- **最適化**: プレイヤーバスト時のディーラーターンスキップ

### 🎨 ユーザーインターフェース

- **日本語完全対応**: メニュー、メッセージ、ヘルプ全て日本語
- **直感的な操作**: 矢印キー + Enterでの選択
- **リアルタイム表示**: カードの数値表示（J(10), Q(10), K(10), A(11)）
- **動的エース表示**: バスト回避時のA(1)表示

### ⚙️ 設定システム

- **通貨設定**: `game_config.toml` で通貨名・通貨フルネーム設定可能
- **ベット額**: 設定ファイルで初期ベット額指定
- **プレイヤー資金**: 初期残高設定

### 🏗️ アーキテクチャ

- **モジュール化**: 機能ごとに分離されたクリーンな構造
  - `card.rs`: カード・デッキ管理
  - `menu.rs`: UIメニュー管理
  - `game_action.rs`: ゲームアクション定義
  - `blackjack.rs`: コアルール実装
  - `game.rs`: メインゲーム制御
- **型安全性**: Rustの強力な型システムを活用
- **エラーハンドリング**: `anyhow` による包括的なエラー処理

## 技術仕様

### 依存関係

```toml
[dependencies]
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
dialoguer = "0.11"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
```

### ビルド・実行

```bash
# ビルド
cargo build

# 実行
cargo run

# ヘルプ表示
cargo run -- --help
```

### 設定ファイル

```toml
# game_config.toml
player_starting_bank = 1000
bet_amount = 10
currency_name = "np"
currency_full_name = "nanai points"
player_edge = 0.0
```

## ゲームルール

### 基本ルール

- プレイヤーとディーラーがそれぞれ2枚のカードを受け取る
- ディーラーの1枚目は隠された状態で表示
- プレイヤーは21に近づけるようカードを引く（ヒット）か、現在の手札で勝負（スタンド）
- 21を超えるとバスト（負け）
- ディーラーは16以下でヒット、17以上でスタンド

### 特殊ルール

- **ブラックジャック**: 最初の2枚で21（例: A+10）- 通常の21より強い
- **ダブルダウン**: 最初の2枚でのみ可能、ベット額2倍で1枚のみ引く
- **スプリット**: 同じ値のカードでのみ可能、手札を2つに分割
- **サレンダー**: 最初の2枚でのみ可能、ベット額の半額を返却して降参

### 勝敗判定

1. ブラックジャック > 通常の21
2. 21以下でディーラーより高い数値 = 勝ち
3. 両方同じ数値 = 引き分け（エッジ設定による）
4. 21超え = バスト（負け）

## 開発情報

### プロジェクト構造

```text
src/
├── main.rs          # エントリーポイント
├── config.rs        # 設定管理
├── card.rs          # カード・デッキ管理
├── menu.rs          # UIメニュー
├── game_action.rs   # ゲームアクション定義
├── blackjack.rs     # ブラックジャックコアルール
└── game.rs          # メインゲーム制御
```

### バージョン履歴

- **v0.1.0**: 基本ブラックジャック実装
- **v0.2.0**: 高度なルール追加（ダブルダウン、スプリット、サレンダー）

## ライセンス

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

```
src/
├── main.rs          # エントリーポイント
├── config.rs        # 設定管理
├── card.rs          # カード・デッキ管理
├── menu.rs          # UIメニュー
├── game_action.rs   # ゲームアクション定義
├── blackjack.rs     # ブラックジャックコアルール
└── game.rs          # メインゲーム制御
```

### バージョン履歴

- **v0.1.0**: 基本ブラックジャック実装
- **v0.2.0**: 高度なルール追加（ダブルダウン、スプリット、サレンダー）

## ライセンス

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 貢献

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 連絡先

Project Link: [https://github.com/nanaisisi/nanai_veintiuna_rs](https://github.com/nanaisisi/nanai_veintiuna_rs)
