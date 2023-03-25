{ pkgs, ... }:

{
	packages = [ 
		pkgs.git  
	]; 
  # https://devenv.sh/languages/
  languages.rust.enable = true;
}
