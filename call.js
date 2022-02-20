import init from './pkg/gazorepkiller.js';

async function run() {
    await init();
}

// export for entrypoint.js
export function main() {
    run();
}
