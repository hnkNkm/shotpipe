{
  description = "Shotpipe - Screenshot annotation tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # 最新の安定版Rustツールチェーン
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            
            # Tauri開発用 - 最新のNode.js LTS
            nodejs_22  # Node.js 22 (最新LTS)
            pnpm       # 最新のpnpm
            
            # Platform-specific dependencies
          ] ++ lib.optionals stdenv.isLinux [
            webkitgtk_4_1
            gtk3
            libsoup_3
            glib
            cairo
            pango
            atk
            gdk-pixbuf
            librsvg
          ];

          shellHook = ''
            echo "🚀 Shotpipe development environment"
            echo "Rust: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
            echo "Node: $(node --version)"
            echo "pnpm: $(pnpm --version)"
            echo ""
            echo "Run 'cargo init' to initialize Rust project"
          '';
        };
      });
}