# Shotpipe 🚀

スクリーンショット即送信用ミニ注釈ツール - クリップボードの画像に素早く注釈を付けて共有できるデスクトップアプリケーション

## ✨ 特徴

- 📋 **クリップボード監視** - 画像をコピーすると自動検知
- ✏️ **簡単な注釈** - テキストや矢印をサッと追加
- ⌨️ **ショートカット操作** - Ctrl+1/2/3で即座に出力
- 🎯 **送信特化** - 保存不要、共有に最適化されたワークフロー
- 🖥️ **クロスプラットフォーム** - Windows/macOS対応

## 📦 インストール

### リリース版（準備中）
GitHubのReleasesページから最新版をダウンロード予定

### 開発版のビルド
```bash
# リポジトリのクローン
git clone https://github.com/hnkNkm/shotpipe.git
cd shotpipe

# Nix環境を使用（推奨）
nix develop

# 依存関係のインストール
pnpm install

# 開発モードで起動
pnpm tauri dev

# リリースビルド
pnpm tauri build
```

## 🎮 使い方

1. **画像をコピー** - スクリーンショットや画像をクリップボードにコピー
2. **通知をクリック** - 「画像を検知しました」通知から編集画面を開く
3. **注釈を追加** - テキストや矢印で説明を追加
4. **ショートカットで出力**
   - `Ctrl+1` / `Cmd+1`: プリセット1で出力
   - `Ctrl+2` / `Cmd+2`: プリセット2で出力
   - `Ctrl+3` / `Cmd+3`: プリセット3で出力
   - `Esc`: キャンセル

## 🛠️ 技術スタック

- **[Tauri v2](https://tauri.app/)** - デスクトップアプリフレームワーク
- **[Rust](https://www.rust-lang.org/)** - バックエンド処理
- **[React](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/)** - フロントエンドUI
- **[Vite](https://vitejs.dev/)** - ビルドツール

## 📂 プロジェクト構造

```
shotpipe/
├── src/                    # React/TypeScriptフロントエンド
├── src-tauri/             # Rustバックエンド
├── docs/                  # ドキュメント
│   ├── specification/     # 仕様書
│   ├── implementation/    # 実装計画
│   ├── architecture/      # アーキテクチャ
│   └── guides/           # 開発ガイド
└── public/               # 静的アセット
```

## 🚀 開発環境

### 前提条件
- Nix（推奨）または
  - Rust 1.75+
  - Node.js 22 LTS
  - pnpm

### セットアップ
```bash
# Nix環境に入る
nix develop

# 開発サーバーの起動
pnpm tauri dev
```

詳細は[開発ガイド](docs/guides/development.md)を参照してください。

## 📋 MVP機能

### 実装済み
- [ ] クリップボード画像の監視
- [ ] 通知システム
- [ ] テキスト注釈
- [ ] 矢印注釈
- [ ] Undo機能
- [ ] プリセット機能
- [ ] システムトレイ

### 今後の予定
- モザイク/ぼかし機能
- 図形注釈（枠・ハイライト）
- 履歴管理
- 複数画像の編集

## 🤝 コントリビューション

Issue報告やPull Requestは[Issues](https://github.com/hnkNkm/shotpipe/issues)へ。
開発参加前に[開発ガイド](docs/guides/development.md)を参照。

## 📝 ライセンス

MIT License

---

**開発状況**: 🚧 MVP開発中