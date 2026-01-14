{
  description = "Rust dev environment with robust Windows cross-compilation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        overrides = (builtins.fromTOML (builtins.readFile (self + "/rust-toolchain.toml")));
      in
      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = [ 
            pkgs.pkg-config 
          ];

          buildInputs = with pkgs; [
            clang
            llvmPackages.bintools
            llvmPackages.lld
            rustup
          ];

          RUSTC_VERSION = overrides.toolchain.channel;

          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.clang}/bin/clang";
          CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS = "-C link-arg=-fuse-ld=lld";

          # --- 以下是 Linux 原生環境設定 ---
          
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
          
          shellHook = ''
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
          '';

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);

          BINDGEN_EXTRA_CLANG_ARGS =
          (builtins.map (a: ''-I"${a}/include"'') [
            pkgs.glibc.dev
          ])
          ++ [
            ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
            ''-I"${pkgs.glib.dev}/include/glib-2.0"''
            ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          ];
        };
      }
    );
}