set -xe

repository_name="gazorepkiller"

function build() {
    wasm-pack build --target web --features wee_alloc
    rm pkg/${repository_name}.d.ts pkg/${repository_name}_bg.wasm.d.ts
    zip -r -FS ${repository_name}-extension.zip pkg call.js entrypoint.js manifest.json 
}

function create_source() {
    zip -r -FS docs/amo/${repository_name}-source.zip . --exclude '*.git*' '*.zip' './README.md' './manifest.json' './target*' './docs/*.png'
}

[[ $# = 0 ]] && build || create_source
