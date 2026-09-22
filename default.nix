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
    hash = "sha256-LgQXB006YeurnqiLhDjfv5PjI0murz/IoHUF2ogYCWM=";
  };

  cargoHash = "sha256-rWvxzRmRr3BJDYOA+ZhlThpTtnA/Ip5X/EYnvrr3iLg=";

  meta = {
    description = "CLI tool to see file size";
    homepage = "https://github.com/NyxInTime/sizeof";
    liscense = lib.licenses.mit;
    maintainers = [ ];
  };

})
