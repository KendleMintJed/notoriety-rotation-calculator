{pkgs, ...}: {
  packages = with pkgs; [trunk];
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  processes.trunk.exec = "trunk serve";
}
