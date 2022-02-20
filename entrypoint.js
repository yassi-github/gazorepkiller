// https://github.com/otiai10/chrome-extension-es6-import
(async () => {
    const src = browser.runtime.getURL('call.js');
    const contentScript = await import(src);
    contentScript.main(/* chrome: no need to pass it */);
})();
