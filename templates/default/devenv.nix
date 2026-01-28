{ pkgs, lib, inputs, ... }:

let
  system = pkgs.stdenv.hostPlatform.system;
in
{
  # Import Decision Graph integration
  imports = [ inputs.dg.devenvModules.default ];

  # Include the dg CLI binary
  packages = [
    inputs.dg.packages.${system}.default
  ];

  # Decision Graph is enabled by default
  # dg.enable = true;

  # Add your project-specific configuration:
  # languages.python.enable = true;
  # packages = [ pkgs.nodejs ];

  enterShell = ''
    echo "Project with Decision Graph"
    echo ""
    echo "Run 'dg --help' for commands"
    echo "Claude skills: /decision, /adr, /incident, /runbook, /meeting"
  '';
}
