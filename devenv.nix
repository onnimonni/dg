{ pkgs, lib, ... }:

{
  # Enable Rust toolchain (uses nixpkgs default)
  languages.rust.enable = true;

  # Additional packages
  packages = with pkgs; [
    graphviz      # For DOT graph visualization
    d2            # For D2 diagram rendering
    python3       # For hooks
    # Fonts (for copying to static folder)
    inter
    jetbrains-mono
  ];

  # Browser automation for testing
  claude.code.mcpServers.playwright = {
    type = "stdio";
    command = "npx";
    args = [ "@playwright/mcp@latest" ];
  };

  # Gemini UX review via consult-llm-mcp (requires GEMINI_API_KEY env var)
  # System prompt configured in ~/.consult-llm-mcp/SYSTEM_PROMPT.md
  claude.code.mcpServers.consult-llm = {
    type = "stdio";
    command = "npx";
    args = [ "-y" "consult-llm-mcp" ];
    env = {
      CONSULT_LLM_DEFAULT_MODEL = "gemini-3-pro-preview";
      CONSULT_LLM_ALLOWED_MODELS = "gemini-3-pro-preview,gemini-2.5-pro";
    };
  };

  # Environment variables
  env = {
    RUST_BACKTRACE = "1";
  };

  # Git hooks
  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;

    dg-fmt = {
      enable = true;
      name = "dg fmt";
      entry = "${pkgs.writeShellScript "dg-fmt" ''
        if [ -x ./target/release/dg ]; then
          ./target/release/dg fmt --check
        elif command -v dg &> /dev/null; then
          dg fmt --check
        fi
      ''}";
      files = "\\decisions/.*\\.md$";
      pass_filenames = false;
    };

    dg-lint = {
      enable = true;
      name = "dg lint";
      entry = "${pkgs.writeShellScript "dg-lint" ''
        if [ -x ./target/release/dg ]; then
          ./target/release/dg lint
        elif command -v dg &> /dev/null; then
          dg lint
        fi
      ''}";
      files = "\\decisions/.*\\.md$";
      pass_filenames = false;
    };
  };

  # Scripts
  scripts = {
    build.exec = "cargo build";
    release.exec = "cargo build --release";
    test.exec = "cargo test";
    install.exec = "cargo install --path .";

    # CSS build commands (uses Tailwind v4 standalone for DaisyUI support)
    css-build.exec = "$DEVENV_ROOT/src/serve/static/tailwindcss -i src/serve/static/input.css -o src/serve/static/tailwind.css --minify";
    css-watch.exec = "$DEVENV_ROOT/src/serve/static/tailwindcss -i src/serve/static/input.css -o src/serve/static/tailwind.css --watch";

    # Full build (CSS + release binary)
    build-all.exec = ''
      echo "Building CSS..."
      "$DEVENV_ROOT/src/serve/static/tailwindcss" -i src/serve/static/input.css -o src/serve/static/tailwind.css --minify
      echo "Building release binary..."
      cargo build --release
    '';

    # dg alias - uses release build if available, falls back to debug
    dg.exec = ''
      if [ -x "$DEVENV_ROOT/target/release/dg" ]; then
        "$DEVENV_ROOT/target/release/dg" "$@"
      elif [ -x "$DEVENV_ROOT/target/debug/dg" ]; then
        "$DEVENV_ROOT/target/debug/dg" "$@"
      else
        echo "dg not built. Run: cargo build"
        exit 1
      fi
    '';
  };

  # ============================================================================
  # Claude Code Integration
  # ============================================================================

  claude.code.enable = true;

  # Permissions
  claude.code.permissions = {
    defaultMode = "default";

    rules = {
      Bash = {
        allow = [
          "dg:*"           # All dg commands
          "d2:*"           # D2 diagram rendering
          "cargo:*"        # Rust build
          "git:*"          # Git operations
          "ls:*"
          "cat:*"
        ];
        deny = [
          "rm -rf:*"
          "sudo:*"
        ];
      };
    };
  };

  # Hooks
  claude.code.hooks = {
    dg-session-start = {
      enable = true;
      hookType = "Notification";
      command = "python3 \"$CLAUDE_PROJECT_DIR\"/.claude/hooks/session-start.py";
    };

    dg-session-stop = {
      enable = true;
      hookType = "Stop";
      command = "python3 \"$CLAUDE_PROJECT_DIR\"/.claude/hooks/session-stop.py";
    };
  };

  # Slash commands
  claude.code.commands = {
    dg-stats = "dg stats";
    dg-graph = "dg graph";
    dg-list = "dg list";
    dg-principles = "dg principles";
    dg-suggest = "dg suggest";
    dg-serve = "dg serve --open";
  };

  # Shell hook
  enterShell = ''
    echo "Decision Graph Development Environment"
    echo ""
    echo "Build commands: build, release, test, install"
    echo "CSS commands: css-build, css-watch"
    echo ""
    echo "dg commands:"
    echo "  dg new <type> <title>  - Create record"
    echo "  dg list                - List records"
    echo "  dg search <query>      - Search records"
    echo "  dg graph               - Show graph"
    echo "  dg principles          - List foundational records"
    echo "  dg why <id>            - Trace dependencies"
    echo "  dg impact <id>         - Show dependents"
    echo "  dg context <topic>     - LLM-friendly context"
    echo "  dg suggest             - Suggest missing decisions"
    echo "  dg serve               - Start HTTP server"
    echo "  dg build               - Generate static site"
    echo ""
    echo "Claude skills: /decision, /adr, /incident, /runbook, /meeting, /context, /frontend"
    echo ""
    echo "MCP servers: playwright (browser), consult-llm (Gemini UX review - needs GEMINI_API_KEY)"

    # Copy fonts from Nix store to static folder for rust-embed
    mkdir -p "$DEVENV_ROOT/src/serve/static/fonts"
    cp ${pkgs.inter}/share/fonts/truetype/InterVariable.ttf "$DEVENV_ROOT/src/serve/static/fonts/" 2>/dev/null || true
    cp ${pkgs.jetbrains-mono}/share/fonts/WOFF2/JetBrainsMono-Regular.woff2 "$DEVENV_ROOT/src/serve/static/fonts/" 2>/dev/null || true

    # Download Tailwind CSS v4 standalone (required for DaisyUI v5)
    if [ ! -f "$DEVENV_ROOT/src/serve/static/tailwindcss" ]; then
      echo "Downloading Tailwind CSS v4 standalone..."
      ARCH=$(uname -m)
      case "$ARCH" in
        arm64|aarch64) BINARY="tailwindcss-macos-arm64" ;;
        x86_64) BINARY="tailwindcss-macos-x64" ;;
        *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
      esac
      curl -sL "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/$BINARY" -o "$DEVENV_ROOT/src/serve/static/tailwindcss"
      chmod +x "$DEVENV_ROOT/src/serve/static/tailwindcss"
    fi

    # Download DaisyUI standalone bundles if not present
    if [ ! -f "$DEVENV_ROOT/src/serve/static/daisyui.mjs" ]; then
      echo "Downloading DaisyUI bundle..."
      curl -sL "https://github.com/saadeghi/daisyui/releases/latest/download/daisyui.mjs" -o "$DEVENV_ROOT/src/serve/static/daisyui.mjs"
      curl -sL "https://github.com/saadeghi/daisyui/releases/latest/download/daisyui-theme.mjs" -o "$DEVENV_ROOT/src/serve/static/daisyui-theme.mjs"
    fi
  '';
}
