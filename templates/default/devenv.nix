{ pkgs, lib, inputs, ... }:

{
  # Import Decision Graph integration
  imports = [ "${inputs.dg}/devenv-module.nix" ];

  # Decision Graph is enabled by default
  # Customize settings below:

  # dg.enable = true;  # Enabled by default

  # Add your project-specific configuration:
  # languages.python.enable = true;
  # packages = [ pkgs.nodejs ];

  enterShell = ''
    echo "Project with Decision Graph"
    echo ""
    echo "Decision Graph commands:"
    echo "  dg new <type> <title>  - Create record"
    echo "  dg list                - List records"
    echo "  dg search <query>      - Search"
    echo "  dg graph               - View graph"
    echo ""
    echo "Claude skills: /decision, /adr, /incident, /runbook, /meeting"
  '';
}
