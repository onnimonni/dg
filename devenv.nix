{ pkgs, lib, ... }:

{
  # Enable Rust toolchain
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # Additional packages
  packages = with pkgs; [
    graphviz  # For graph visualization
    python3   # For hooks
  ];

  # Environment variables
  env = {
    RUST_BACKTRACE = "1";
  };

  # Git hooks
  pre-commit.hooks = {
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
      hookType = "SessionStart";
      command = "python3 \"$CLAUDE_PROJECT_DIR\"/.claude/hooks/session-start.py";
      timeout = 10000;
    };

    dg-session-stop = {
      hookType = "Stop";
      command = "python3 \"$CLAUDE_PROJECT_DIR\"/.claude/hooks/session-stop.py";
      timeout = 5000;
    };
  };

  # Slash commands
  claude.code.commands = {
    dg-stats = "dg stats";
    dg-graph = "dg graph";
    dg-list = "dg list";
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
    echo ""
    echo "Claude skills: /decision, /adr, /incident, /runbook, /meeting, /context"
  '';
}
