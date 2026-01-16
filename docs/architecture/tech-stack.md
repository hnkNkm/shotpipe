# Shotpipe 技術スタック詳細

## アーキテクチャ概要

Shotpipeは、Tauri v2を基盤とした デスクトップアプリケーションです。Rustでネイティブ機能を実装し、ReactでUIを構築する構成を採用しています。

```
┌─────────────────────────────────────┐
│         Frontend (React)             │
│  - UI Components                     │
│  - Canvas Editor                     │
│  - State Management                  │
└──────────────┬──────────────────────┘
               │ Tauri IPC
┌──────────────▼──────────────────────┐
│         Backend (Rust)               │
│  - Clipboard Monitor                 │
│  - Image Processing                  │
│  - System Tray                       │
│  - Notification                      │
└─────────────────────────────────────┘
```

## Core Technologies

### Tauri v2
- **バージョン**: 2.x (最新安定版)
- **役割**: アプリケーションフレームワーク
- **選定理由**:
  - 軽量で高速
  - クロスプラットフォーム対応
  - セキュアなアーキテクチャ
  - Rust + Web技術の組み合わせ

### Rust (Backend)
- **バージョン**: 1.75+ (最新安定版)
- **役割**: ネイティブ機能の実装
- **主要機能**:
  - システムレベルの機能実装
  - パフォーマンスクリティカルな処理
  - 安全なメモリ管理

### React + TypeScript (Frontend)
- **React**: 18.2+
- **TypeScript**: 5.0+
- **役割**: ユーザーインターフェース
- **選定理由**:
  - コンポーネントベースの開発
  - 型安全性
  - 豊富なエコシステム

## Rust Dependencies

### 必須クレート

```toml
[dependencies]
# Tauri Core
tauri = { version = "2", features = ["system-tray", "notification"] }
tauri-build = { version = "2" }

# クリップボード操作
arboard = "3.4"

# 画像処理
image = "0.25"

# シリアライゼーション
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 非同期処理
tokio = { version = "1", features = ["full"] }

# ログ出力
log = "0.4"
env_logger = "0.11"

# Base64エンコーディング
base64 = "0.22"
```

### Tauriプラグイン

```toml
[dependencies]
# クリップボード監視
tauri-plugin-clipboard = "2.0"

# システム通知
tauri-plugin-notification = "2.0"

# システムトレイ
tauri-plugin-system-tray = "2.0"

# 設定の永続化
tauri-plugin-store = "2.0"

# ファイルダイアログ（将来拡張用）
tauri-plugin-dialog = "2.0"
```

## Frontend Dependencies

### パッケージ一覧

```json
{
  "dependencies": {
    // Tauri API
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-clipboard": "^2.0.0",
    "@tauri-apps/plugin-notification": "^2.0.0",
    
    // React
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    
    // UI Components
    "lucide-react": "^0.400.0",
    
    // キーボードショートカット
    "react-hotkeys-hook": "^4.5.0",
    
    // 状態管理
    "zustand": "^4.5.0",
    
    // ユーティリティ
    "clsx": "^2.1.0"
  },
  "devDependencies": {
    // TypeScript
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "typescript": "^5.0.0",
    
    // ビルドツール
    "vite": "^5.0.0",
    "@vitejs/plugin-react": "^4.0.0",
    
    // Linting
    "eslint": "^8.57.0",
    "@typescript-eslint/eslint-plugin": "^7.0.0",
    "@typescript-eslint/parser": "^7.0.0",
    
    // Formatting
    "prettier": "^3.0.0"
  }
}
```

## 開発環境

### Nix Flake設定

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Rust
          rust-bin.stable.latest.default
          
          # Node.js
          nodejs_22
          pnpm
          
          # システム依存
          pkg-config
          openssl
          
          # macOS specific
          darwin.apple_sdk.frameworks.AppKit
          darwin.apple_sdk.frameworks.WebKit
        ];
      };
    };
}
```

## API設計

### Tauri Commands

```rust
// クリップボード関連
#[tauri::command]
async fn get_clipboard_image() -> Result<String, String>;

#[tauri::command]
async fn set_clipboard_image(image_data: String) -> Result<(), String>;

// 注釈処理
#[tauri::command]
async fn process_image_with_annotations(
    image_data: String,
    annotations: Vec<Annotation>,
    preset: Preset
) -> Result<String, String>;

// 設定管理
#[tauri::command]
async fn get_settings() -> Result<Settings, String>;

#[tauri::command]
async fn save_settings(settings: Settings) -> Result<(), String>;

// 監視制御
#[tauri::command]
async fn start_monitoring() -> Result<(), String>;

#[tauri::command]
async fn stop_monitoring() -> Result<(), String>;
```

### データ構造

```typescript
// 注釈
interface Annotation {
  type: 'text' | 'arrow';
  position: { x: number; y: number };
  data: TextData | ArrowData;
}

interface TextData {
  content: string;
  size: 'small' | 'medium' | 'large';
  color: string;
}

interface ArrowData {
  endPosition: { x: number; y: number };
  color: string;
  thickness: number;
}

// プリセット
interface Preset {
  id: string;
  name: string;
  maxWidth: number;
  format: 'PNG';
  postAction: 'clipboard';
  closeAfterCopy: boolean;
}

// 設定
interface Settings {
  monitoring: boolean;
  presets: Preset[];
  annotationDefaults: {
    textSize: 'small' | 'medium' | 'large';
    color: string;
  };
}
```

## パフォーマンス考慮事項

### 画像処理
- 大きい画像（4K以上）のリサイズ処理
- メモリ効率的なBase64エンコード/デコード
- Canvas描画の最適化

### クリップボード監視
- ポーリング間隔: 500ms
- デバウンス処理: 1秒
- バックグラウンドスレッドでの実行

### メモリ管理
- 画像データの適切な解放
- 操作履歴の制限（Undo最大10回）
- 不要なレンダリングの回避

## セキュリティ考慮事項

### Tauri Security
- CSP (Content Security Policy) の適切な設定
- IPC通信の検証
- 最小権限の原則

### データ処理
- クリップボードデータの検証
- 画像フォーマットの検証
- サイズ制限（最大100MB）

## テスト戦略

### Unit Tests
- Rust: `cargo test`
- React: `vitest`

### Integration Tests
- Tauri E2E: `tauri-driver`
- UI Tests: `playwright`

### 手動テスト項目
- クリップボード監視の動作確認
- 通知の表示確認
- ショートカットキーの動作確認
- 各OS固有機能の確認

## CI/CD

### GitHub Actions

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: pnpm/action-setup@v3
      - name: Install dependencies
        run: pnpm install
      - name: Rust tests
        run: cargo test
      - name: Build
        run: pnpm tauri build
```

## デプロイメント

### ビルド成果物
- **Windows**: `.msi`, `.exe`
- **macOS**: `.dmg`, `.app`
- **Linux**: `.AppImage`, `.deb`

### 配布方法
- GitHub Releases
- 自動更新機能（tauri-plugin-updater）

## 将来の拡張性

### 検討中の技術
- **WebAssembly**: 画像処理の高速化
- **WebGPU**: Canvas描画の高速化
- **Machine Learning**: 自動注釈機能
- **Cloud Sync**: 設定の同期機能