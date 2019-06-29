let
    pkgsSrc = fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/8c09536ef10e72c9421c05403c68134480ba7fde.tar.gz";
        sha256 = "1kf8dl583lbabw93lpc4m5f62pqsip870d8n8hgw06s8898l7mkp";
    };
    pkgsConfig = {};
    pkgs = import pkgsSrc {config = pkgsConfig;};
in
    pkgs.stdenv.mkDerivation {
        name = "concrete";
        buildInputs = [pkgs.cargo];
        phases = ["configurePhase"];
        configurePhase = ''
            2>&1 echo 'This derivation is only for use with nix-shell.'
            false
        '';
    }
