{ pkgs, lib, ... }:

{
  # Enable Rust toolchain
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # Additional packages
  packages = with pkgs; [
    graphviz  # For DOT graph visualization
    d2        # For D2 diagram rendering
    python3   # For hooks
  ];

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
      files = "\\.decisions/.*\\.md$";
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
      files = "\\.decisions/.*\\.md$";
      pass_filenames = false;
    };
  };

  # Scripts
  scripts = {
    build.exec = "cargo build";
    release.exec = "cargo build --release";
    test.exec = "cargo test";
    install.exec = "cargo install --path .";
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
    echo "Claude skills: /decision, /adr, /incident, /runbook, /meeting, /context"
  '';
}
