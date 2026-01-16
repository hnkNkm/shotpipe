# Shotpipe 開発ガイド

## 開発環境のセットアップ

### 前提条件

- Git
- Nix (推奨) または 個別インストール
- macOS または Windows 10/11

### Nixを使用する場合（推奨）

1. **Nix環境に入る**
```bash
nix develop
```

これにより、以下がセットアップされます：
- Rust (最新安定版)
- Node.js 22 LTS
- pnpm
- 必要なシステム依存ライブラリ

### 手動セットアップの場合

1. **Rustのインストール**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

2. **Node.jsのインストール**
```bash
# Node.js 22 LTSをインストール
# https://nodejs.org/ からダウンロード
```

3. **pnpmのインストール**
```bash
npm install -g pnpm
```

4. **システム依存の準備**

macOS:
```bash
xcode-select --install
```

Windows:
- Visual Studio Build Tools 2022
- WebView2 (Windows 11では標準搭載)

## プロジェクトのセットアップ

### 1. リポジトリのクローン
```bash
git clone https://github.com/your-org/shotpipe.git
cd shotpipe
```

### 2. 依存関係のインストール
```bash
# Nix環境に入る（Nix使用時）
nix develop

# フロントエンドの依存関係
pnpm install

# Rust依存関係（自動でダウンロード）
cargo build
```

### 3. 開発サーバーの起動
```bash
pnpm tauri dev
```

これにより：
- Viteの開発サーバーが起動（http://localhost:1420）
- Rustのコンパイルが実行
- Tauriアプリケーションウィンドウが開く

## 開発ワークフロー

### コード変更時の挙動

- **React/TypeScript**: Hot Module Replacement (HMR) により自動リロード
- **Rust**: 変更検知により自動再コンパイル＆再起動
- **Tauri設定**: アプリの再起動が必要

### よく使うコマンド

```bash
# 開発サーバー起動
pnpm tauri dev

# ビルド（デバッグ版）
pnpm tauri build --debug

# ビルド（リリース版）
pnpm tauri build

# Rustのテスト実行
cargo test

# Rustのフォーマット
cargo fmt

# Rustのlint
cargo clippy

# TypeScriptの型チェック
pnpm tsc --noEmit

# フロントエンドのlint
pnpm lint

# フロントエンドのフォーマット
pnpm format
```

## プロジェクト構造

```
shotpipe/
├── src/                    # React/TypeScriptフロントエンド
│   ├── components/        # UIコンポーネント
│   ├── hooks/            # カスタムフック
│   ├── lib/              # ユーティリティ関数
│   ├── stores/           # 状態管理
│   ├── types/            # TypeScript型定義
│   ├── App.tsx           # メインアプリコンポーネント
│   └── main.tsx          # エントリーポイント
├── src-tauri/             # Rustバックエンド
│   ├── src/
│   │   ├── clipboard/    # クリップボード処理
│   │   ├── image/        # 画像処理
│   │   ├── commands/     # Tauriコマンド
│   │   ├── tray/         # システムトレイ
│   │   ├── lib.rs        # ライブラリエントリー
│   │   └── main.rs       # メインエントリー
│   ├── Cargo.toml        # Rust依存関係
│   └── tauri.conf.json   # Tauri設定
├── public/                # 静的アセット
├── docs/                  # ドキュメント
└── package.json          # Node依存関係
```

## 開発のベストプラクティス

### 1. コミット規約

```
feat: 新機能追加
fix: バグ修正
docs: ドキュメント変更
style: フォーマット変更
refactor: リファクタリング
test: テスト追加/修正
chore: ビルド/補助ツール変更
```

例:
```bash
git commit -m "feat: テキスト注釈機能を追加"
git commit -m "fix: クリップボード監視のメモリリークを修正"
```

### 2. ブランチ戦略

```
main           # 本番環境
├── develop    # 開発環境
    ├── feature/text-annotation
    ├── feature/arrow-tool
    └── fix/clipboard-memory-leak
```

### 3. コードスタイル

**Rust:**
- `cargo fmt`でフォーマット
- `cargo clippy`でlint
- 警告は0を維持

**TypeScript:**
- ESLint + Prettierでフォーマット
- 型安全性を重視
- `any`型の使用を避ける

### 4. エラーハンドリング

**Rust側:**
```rust
#[tauri::command]
async fn process_image(data: String) -> Result<String, String> {
    match image_processor::process(&data) {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("Image processing failed: {}", e);
            Err(format!("画像処理に失敗しました: {}", e))
        }
    }
}
```

**React側:**
```typescript
try {
  const result = await invoke('process_image', { data: imageData });
  // 成功処理
} catch (error) {
  console.error('Failed to process image:', error);
  showNotification({ type: 'error', message: '画像処理に失敗しました' });
}
```

## デバッグ

### 開発者ツール

開発モードでは、以下のショートカットで開発者ツールを開く：
- Windows/Linux: `Ctrl + Shift + I`
- macOS: `Cmd + Option + I`

### Rustのデバッグ

```rust
// デバッグ出力
log::debug!("Debug message: {:?}", variable);
log::info!("Info message");
log::error!("Error message");

// 環境変数でログレベル設定
RUST_LOG=debug pnpm tauri dev
```

### React DevTools

1. Chrome拡張機能をインストール
2. 開発者ツール内で利用可能

## トラブルシューティング

### よくある問題と解決方法

**1. ビルドエラー: "cannot find -lssl"**
```bash
# macOS
brew install openssl pkg-config

# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config
```

**2. Nix環境でpnpmが見つからない**
```bash
# flake.lockを削除して再度環境に入る
rm flake.lock
nix develop
```

**3. Tauri devでウィンドウが開かない**
```bash
# キャッシュクリア
cargo clean
rm -rf node_modules
pnpm install
pnpm tauri dev
```

**4. ホットリロードが効かない**
- Viteの設定を確認
- `vite.config.ts`でHMRが有効になっているか確認

## テスト

### Rustユニットテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_resize() {
        let image = create_test_image();
        let resized = resize_image(&image, 800);
        assert_eq!(resized.width(), 800);
    }
}
```

実行:
```bash
cargo test
```

### Reactコンポーネントテスト

```typescript
import { render, screen } from '@testing-library/react';
import { TextAnnotation } from './TextAnnotation';

test('renders text annotation', () => {
  render(<TextAnnotation text="Test" />);
  expect(screen.getByText('Test')).toBeInTheDocument();
});
```

実行:
```bash
pnpm test
```

### E2Eテスト

```typescript
// Playwright使用例
import { test, expect } from '@playwright/test';

test('clipboard monitoring', async ({ page }) => {
  await page.goto('/');
  // クリップボードに画像をコピー
  await page.evaluate(() => {
    // テストコード
  });
  // 通知が表示されることを確認
  await expect(page.locator('.notification')).toBeVisible();
});
```

## リリース

### ビルド手順

1. **バージョン更新**
```bash
# package.jsonとCargo.tomlのバージョンを更新
pnpm version patch  # または minor, major
```

2. **リリースビルド**
```bash
pnpm tauri build
```

3. **成果物の場所**
- Windows: `src-tauri/target/release/bundle/msi/`
- macOS: `src-tauri/target/release/bundle/dmg/`

### 署名（macOS）

```bash
# 開発者証明書が必要
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: Your Name" \
  src-tauri/target/release/bundle/macos/Shotpipe.app
```

### 署名（Windows）

```powershell
# 証明書を使用して署名
signtool sign /f certificate.pfx /p password \
  /t http://timestamp.server.com \
  src-tauri/target/release/bundle/msi/Shotpipe.msi
```

## 参考リンク

- [Tauri公式ドキュメント](https://tauri.app)
- [Rust Book](https://doc.rust-lang.org/book/)
- [React公式ドキュメント](https://react.dev)
- [TypeScript公式ドキュメント](https://www.typescriptlang.org)
- [Canvas API](https://developer.mozilla.org/docs/Web/API/Canvas_API)