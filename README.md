# nanai_veintiuna_rs

メニュー駆動式ブラックジャックゲーム（Rust実装）

## 概要

日本語インターフェースを持つブラックジャックゲームです。矢印キーで操作するインタラクティブメニューシステムを採用し、設定ファイルによるカスタマイズが可能です。

## 特徴

- **完全日本語対応**: すべての操作とメッセージが日本語
- **矢印キーナビゲーション**: 直感的なメニュー操作
- **設定ファイル対応**: 通貨名やゲームバランスを自由に設定
- **コマンドライン対応**: メニューをスキップした直接起動も可能

## クイックスタート

```bash
# ビルド
cargo build --release

# 実行
cargo run
```

## 使用方法

### 基本操作
- **矢印キー**: メニューの選択肢を移動
- **Enter**: 選択を確定
- **ゲーム中**: `h`（ヒット）または `s`（スタンド）

### コマンドラインオプション
```bash
cargo run -- --direct        # メニューをスキップして直接ゲーム開始
cargo run -- --help          # ヘルプ表示
cargo run -- --config FILE   # カスタム設定ファイル使用
```

## 設定

`game_config.toml` で通貨や初期資金などを設定できます：

```toml
currency_name = "np"          # 通貨の短縮表示
currency_full_name = "nanai points"  # 通貨の正式名称
player_starting_bank = 1000  # 初期資金
bet_amount = 10              # ベット額
player_edge = 0.05           # プレイヤー有利度
```

## システム要件

- Rust 1.70以上
- Windows/macOS/Linux

## ドキュメント

- `docs/` - 設計文書とゲーム戦略ガイド
- `docs/std_move.md` - ブラックジャック基本戦略
- `docs/update_changes.md` - 開発履歴

## ライセンス

MIT License または Apache License 2.0
