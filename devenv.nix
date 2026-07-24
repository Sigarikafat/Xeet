{
  pkgs,
  lib,
  config,
  ...
}:
{
  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/packages/
  packages = [
    pkgs.openssl
  ];

  # See full reference at https://devenv.sh/reference/options/
}
