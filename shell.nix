let
  system = import <nixpkgs> {};
  moz_overlay = import (
    builtins.fetchTarball {
        url = https://github.com/mozilla/nixpkgs-mozilla/archive/e912ed483e980dfb4666ae0ed17845c4220e5e7c.tar.gz;
        sha256 = "08fvzb8w80bkkabc1iyhzd15f4sm7ra10jn32kfch5klgl0gj3j3";
    }
  );
  nixpkgs = import (
    builtins.fetchTarball {
        url = https://github.com/NixOS/nixpkgs/archive/release-20.03.tar.gz;
        sha256 =
            if system.pkgs.stdenv.isDarwin then
                "0kgqa1axq4lnj5c98lmzrajzciqbj7d7m0g5cn8mi2grb5rr3cca"
            else
                "0b9s1v4ydyb8wsmzccxzvp1agpc6zh96w1mg6j7b6r3b1r1zqcpk";
    }
  ) {
    overlays = [ moz_overlay ];
    config = { allowUnfree = true; };
  };
  frameworks = nixpkgs.darwin.apple_sdk.frameworks;
  rustChannels = (
    nixpkgs.rustChannelOf {
      date = "2020-06-18";
      channel = "stable";
    }
  ); # UPDATE: './rust-toolchain' to reflect on heroku
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "encrypted_id-env";
    buildInputs = [ rustChannels.rust rustChannels.rust-src rustChannels.clippy-preview ];

    nativeBuildInputs = [
      stdenv.cc.cc.lib
      clang
      llvm
      zsh
      locale
      vim
      less
      bat
      git
      gitAndTools.diff-so-fancy
      openssl
      pkgconfig
      perl
      nixpkgs-fmt
      cacert
      libiconv
      python38
    ]
    ++ (
         stdenv.lib.optionals stdenv.isDarwin [
           frameworks.CoreServices
           frameworks.Security
           frameworks.CoreFoundation
           frameworks.Foundation
           frameworks.AppKit
         ]
    );

    RUST_BACKTRACE = 1;
    SOURCE_DATE_EPOCH = 315532800;

    shellHook = (
      if pkgs.stdenv.isDarwin then
        ''export NIX_LDFLAGS="-F${frameworks.AppKit}/Library/Frameworks -framework AppKit -F${frameworks.CoreServices}/Library/Frameworks -framework CoreServices -F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";''
      else
        ""
    )
      +
    ''
      export LD_LIBRARY_PATH=$(rustc --print sysroot)/lib:${stdenv.cc.cc.lib}/lib:$LD_LIBRARY_PATH;
      export LIBCLANG_PATH="${llvmPackages.libclang}/lib"
      export ZDOTDIR=`pwd`;
      export HISTFILE=~/.zsh_history
      export CARGO_TARGET_DIR=`pwd`/target-nix
      echo "Using ${rustChannels.rust.name}."
      unset MACOSX_DEPLOYMENT_TARGET
    '';
  }
