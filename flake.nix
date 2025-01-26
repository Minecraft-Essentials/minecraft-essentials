{
	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		flake-parts.url = "github:hercules-ci/flake-parts";
		agenix-shell.url = "github:aciceri/agenix-shell";
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs =
		inputs@{
			self,
			nixpkgs,
			flake-parts,
			agenix-shell,
			...
		}:
	flake-parts.lib.mkFlake { inherit inputs; } {
		systems = nixpkgs.lib.systems.flakeExposed;

		imports = [
# agenix-shell.flakeModules.default

		];

# agenix-shell = {
#   secrets = {
#     foo.file = ./secrets/foo.age;
#   };
# };

		perSystem =
		{
			pkgs,
			config,
			system,
			lib,
			...
		}:
		let
			toolchain = pkgs.rustPlatform;
		in
		{
			_module.args.pkgs = import nixpkgs {
				inherit system;
				overlays = [
					(inputs.fenix.overlays.default)
				];
			};

      devShells.default = with pkgs; let 
        toolchain = pkgs.fenix.stable.withComponents [
          "rustc"
          "cargo"
          "clippy"
        ];
        in mkShell
        {
          packages = with pkgs; [
            openssl
						hyperfine
            rust-analyzer
						bun # needed for node
            toolchain
          ];
        };
      };
	};
}
