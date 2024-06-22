#!/bin/bash

# usage:
#   build:
#       ./build.sh
#   create source archive:
#       ./build.sh a

set -xe

repository_name="gazorepkiller"

cmdnotfound() {
    local cmd
    for cmd; do
        [[ $(which "$cmd") == "" ]] && return 0
    done
    # all commands found
    return 1
}

prerequired() {
    local clang_version=18
    if cmdnotfound pkg-config openssl make zip ; then
        ${sudo} apt update
        ${sudo} apt install -y pkg-config libssl-dev zip \
            lsb-release wget software-properties-common gnupg # for install llvm
    fi
    if cmdnotfound clang-${clang_version} ; then
        curl --proto '=https' --tlsv1.2 -sSf https://apt.llvm.org/llvm.sh | DEBIAN_FRONTEND=noninteractive ${sudo} bash -s - ${clang_version}
    fi
    if cmdnotfound cc ; then
        ln -s /usr/bin/clang-${clang_version} /usr/local/bin/cc
    fi
    if cmdnotfound rustc ; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s - -y
    fi
    rustup target add wasm32-unknown-unknown
    if cmdnotfound wasm-pack ; then
        cargo install wasm-pack --version 0.10.2
    fi
}

function build() {
    wasm-pack build --target web --features wee_alloc
    rm pkg/${repository_name}.d.ts pkg/${repository_name}_bg.wasm.d.ts
    zip -r -FS ${repository_name}-extension.zip pkg call.js entrypoint.js manifest.json 
}

function create_source() {
    zip -r -FS docs/amo/${repository_name}-source.zip . --exclude '*.git*' '*.zip' './README.md' './manifest.json' './target*' './docs/*.png'
}

sudo="sudo"
if [[ $(id -u) = 0 ]]; then
    sudo=""
fi
prerequired
[[ $# = 0 ]] && build || create_source
