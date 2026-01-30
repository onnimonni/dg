{ pkgs, lib, config, ... }:

let
  cfg = config.dg;
in
{
  options.dg = {
    enable = lib.mkEnableOption "Decision Graph integration" // { default = true; };
  };

  config = lib.mkIf cfg.enable {
    packages = [
      pkgs.graphviz
      pkgs.python3
    ];

    # Claude Code integration
    claude.code.enable = true;

    claude.code.permissions = {
      defaultMode = "default";
      rules = {
        Bash = {
          allow = [
            "dg:*"
            "git:*"
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

    claude.code.hooks = {
      dg-session-stop = {
        enable = true;
        hookType = "Stop";
        command = ''
          echo '{"continue": true, "message": "Session ending - consider capturing decisions with: dg new decision \"Title\" or dg new adr \"Title\""}'
        '';
      };
    };

    claude.code.commands = {
      dg-stats = "dg stats";
      dg-graph = "dg graph";
      dg-list = "dg list";
    };

    # Initialize dg if not already done
    enterShell = ''
      if command -v dg &> /dev/null && [ ! -d "docs/decisions" ]; then
        echo "Initializing Decision Graph..."
        dg init 2>/dev/null || true
      fi
    '';

    # Git hooks for decision graph
    git-hooks.hooks = {
      dg-lint = {
        enable = true;
        name = "dg lint";
        entry = "${pkgs.writeShellScript "dg-lint" ''
          if command -v dg &> /dev/null; then
            dg lint 2>/dev/null || true
          fi
        ''}";
        files = "\\decisions/.*\\.md$";
        pass_filenames = false;
      };
    };
  };
}
