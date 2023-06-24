{...}: {
  perSystem = {inputs', ...}: let
    # "stable", "latest", "minimal", "complete"
    channel = "latest";
    fenix-channel = inputs'.fenix.packages.${channel};

    # rust targets
    fenix-targets = with inputs'.fenix.packages.targets; [
      x86_64-unknown-linux-gnu.${channel}.rust-std
      # wasm32-unknown-unknown.${channel}.rust-std
    ];

    fenix-toolchain = inputs'.fenix.packages.combine ([
        fenix-channel.rustc
        fenix-channel.cargo
        fenix-channel.clippy
        fenix-channel.rust-analysis
        fenix-channel.rust-src
        fenix-channel.rustfmt
        fenix-channel.llvm-tools-preview
      ]
      ++ fenix-targets);
  in rec {
    packages = {
      rust-toolchain = fenix-toolchain;
    };
  };
}
