export PATH="$coreutils/bin:$cargo/bin:$gcc/bin"
mkdir $out
echo $cargoFile
echo $out
echo Follow the following guide to use a cargo build - https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md
#cargo build --manifest-path $cargoFile --target-dir $out
#rustc --out-dir $out $src
