{ pkgs, ... }:

{
  # Enable Rust toolchain
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # Additional packages
  packages = with pkgs; [
    # For graph visualization
    graphviz
  ];

  # Environment variables
  env = {
    RUST_BACKTRACE = "1";
  };

  # Git hooks
  pre-commit.hooks = {
    # Rust formatting
    rustfmt.enable = true;

    # Clippy linting
    clippy.enable = true;

    # Decision Graph formatting
    dg-fmt = {
      enable = true;
      name = "dg fmt";
      entry = "${pkgs.writeShellScript "dg-fmt" ''
        # Only run if dg binary exists
        if [ -x ./target/release/dg ]; then
          ./target/release/dg fmt --check
        elif command -v dg &> /dev/null; then
          dg fmt --check
        fi
      ''}";
      files = "\\.decisions/.*\\.md$";
      pass_filenames = false;
    };

    # Decision Graph linting
    dg-lint = {
      enable = true;
      name = "dg lint";
      entry = "${pkgs.writeShellScript "dg-lint" ''
        # Only run if dg binary exists
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

  # Scripts for convenience
  scripts = {
    build.exec = "cargo build";
    release.exec = "cargo build --release";
    test.exec = "cargo test";
    install.exec = "cargo install --path .";
  };

  # Shell hook
  enterShell = ''
    echo "Decision Graph Development Environment"
    echo "Commands: build, release, test, install"
  '';
}
