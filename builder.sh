export PATH="$coreutils/bin:$rustc/bin:$gcc/bin"
mkdir $out
rustc --out-dir $out $src
