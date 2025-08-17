# 開発履歴

**最終更新**: 2025年8月18日

## v0.2.0 - 通貨名設定ファイル対応 (2025年8月18日)

### ✅ 実装完了内容

#### 通貨名の設定ファイル対応

- **GameConfig構造体拡張**
  - `currency_name: String` - 短縮表示用（例: "np", "￥", "円"）
  - `currency_full_name: String` - 正式名称（例: "nanai points", "日本円"）
  - デフォルト値: "np" と "nanai points"

- **設定ファイル統合**
  - `game_config.toml`に通貨設定フィールド追加
  - serde デシリアライゼーション対応
  - 既存設定との互換性維持

- **UI表示の動的化**
  - メニュー画面: 「プレイヤー資金: 1000np (通貨名: nanai points)」
  - ゲーム中: 「現在の残高: 1000np」「ベット額: 10np」  
  - 結果表示: 「獲得: +10np」「損失: -10np」

- **動作確認**
  - "np" ↔ "円" の設定変更テスト完了
  - 設定ファイル変更の即座反映確認

### 技術仕様

- **設定ファイル**: `game_config.toml`
- **構造体**: `GameConfig` in `config.rs`
- **表示ロジック**: `game.rs` の `run_game()` および `run_menu_loop()`

### 技術基盤

- **main.rs**: clap CLI + 日本語ヘルプ
- **game.rs**: dialoguer矢印キーナビ + 完全日本語化
- **config.rs**: 設定ファイル駆動システム
- **game_config.toml**: 通貨・ゲームバランス設定

### 依存関係

- clap 4.5.45 (CLI引数処理)
- dialoguer 0.11.0 (対話式メニュー)  
- serde/toml (設定管理)
- rand (カードシャッフル)
- anyhow (エラーハンドリング)
