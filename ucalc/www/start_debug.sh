set -e
set -x
export RUSTUP_TOOLCHAIN=nightly
export RUSTFLAGS='--cfg getrandom_backend="wasm_js" -Zunstable-options -Cpanic=immediate-abort'
wasm-pack build --no-opt --out-dir www/pkg --target web --debug --no-default-features --features "f64,complex,float_rand,wasm"
ls -l pkg/ucalc_bg.wasm
if [ $# -ne 0 ]; then
    python3 -m http.server 8080
fi
