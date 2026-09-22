{
  lib,
  fetchFromGitHub,
  rustPlatform,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "sizeof";
  version = "0.2.0";

  src = fetchFromGitHub {
    owner = "NyxInTime";
    repo = "sizeof";
    tag = finalAttrs.version;
    hash = "sha256-NnnJ6IqNcdBJZXihlqQfESKl97HgB58uIEHLYym3A/Q=";
  };

  cargoHash = "sha256-VrI3hzEGVm03umCQmxq51w3q3wpld3v5k5K2kjH5cYE=";

  meta = {
    description = "CLI tool to see file size";
    homepage = "https://github.com/NyxInTime/sizeof";
    liscense = lib.licenses.mit;
    maintainers = [ ];
  };

})
