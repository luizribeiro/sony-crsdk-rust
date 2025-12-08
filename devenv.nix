{ pkgs, ... }:

{
  dotenv.enable = true;

  languages.rust.enable = true;

  enterShell = ''
    cargo build -p sonyctl --quiet 2>/dev/null || true
    export PATH="$DEVENV_ROOT/target/debug:$PATH"
  '';

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy = {
      enable = true;
      settings.allFeatures = true;
    };
    cargo-check.enable = true;
  };
}
