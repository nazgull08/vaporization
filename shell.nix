{ pkgs ? import <nixpkgs> {
  config.allowUnfree = true;

} }:

let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
  libxkbcommonPath = "${pkgs.libxkbcommon}/lib";
in


pkgs.mkShell {
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libxkbcommonPath}"
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
      pkgs.alsaLib
      pkgs.udev
      pkgs.vulkan-loader
    ]}"
  '';

  buildInputs = with pkgs; [
    (
      with fenix;
      combine (
        with default; [
          cargo
          clippy-preview
          latest.rust-src
          rust-analyzer
          rust-std
          rustc
          rustfmt-preview
        ]
      )
    )
    cargo-edit
    cargo-watch

    lld
    clang

    # # bevy-specific deps (from https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
    pkg-config
    udev
    alsaLib
    lutris
    
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libxkbcommon

    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers

    wasm-pack
    wasm-bindgen-cli
  ];

}
