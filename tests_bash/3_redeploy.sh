set -e
sh ./1_compile.sh
cd ..
near dev-deploy -f w-near-141/res/w_near.wasm
