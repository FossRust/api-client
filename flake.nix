{
  description = "FossRust API Client dev shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

  outputs = { self, nixpkgs }:
  let
    systems = [ "x86_64-linux" "aarch64-linux" ];
    forAll = nixpkgs.lib.genAttrs systems;
  in {
    devShells = forAll (system:
      let
        pkgs = import nixpkgs { inherit system; };
        libPath = pkgs.lib.makeLibraryPath [
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.xorg.libX11
          pkgs.xorg.libXi
          pkgs.xorg.libXrandr
          pkgs.xorg.libXcursor
          pkgs.mesa
          pkgs.vulkan-loader
          pkgs.fontconfig
          pkgs.freetype
          pkgs.alsa-lib
        ];
      in {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustup pkg-config cmake
            wayland libxkbcommon
            xorg.libX11 xorg.libXi xorg.libXrandr xorg.libXcursor
            mesa vulkan-loader vulkan-tools
            fontconfig freetype
            alsa-lib
            git
          ];

          shellHook = ''
            if [ -z "''${WINIT_UNIX_BACKEND:-}" ]; then
              if [ -n "''${WAYLAND_DISPLAY:-}" ]; then
                export WINIT_UNIX_BACKEND=wayland
              else
                export WINIT_UNIX_BACKEND=x11
              fi
            fi

            export LD_LIBRARY_PATH="${libPath}:''${LD_LIBRARY_PATH:-}"
            export RUST_BACKTRACE=1
            echo "WINIT_UNIX_BACKEND=$WINIT_UNIX_BACKEND"
          '';
        };
      }
    );
  };
}
