function preloadImage(src) {
    return new Promise((res, rej) => {
        let img = document.createElement("img");

        img.onload = () => {
            res();
        }

        img.onerror = (e) => {
            rej(e);
        }

        img.src = src;
    })
}

window.addEventListener('load', async () => {
    await preloadImage("./static/logo/logo.svg");
    await preloadImage("./static/logo/logo-footer-dark.svg");
    await preloadImage("./static/logo/logo-footer-light.svg");
    await preloadImage("./static/logo/powered-by-vercel.svg");
    await preloadImage("./static/icons/status-critical.svg");
    await preloadImage("./static/icons/status-critical-dark.svg");
    await preloadImage("./static/icons/status-info.svg");
    await preloadImage("./static/icons/status-info-dark.svg");
    await preloadImage("./static/icons/status-success.svg");
    await preloadImage("./static/icons/status-success-dark.svg");
    await preloadImage("./static/icons/status-warning.svg");
    await preloadImage("./static/icons/status-warning-dark.svg");
    await preloadImage("./static/hosting/equestriadev.png");
    await preloadImage("./static/hosting/gitbook.png");
    await preloadImage("./static/hosting/ovh.png");
    await preloadImage("./static/hosting/scaleway.png");
    await preloadImage("./static/hosting/vercel.png");

    window.hasDarkTheme = document.getElementById("eqs-dark-theme-indicator").checkVisibility();

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        window.hasDarkTheme = document.getElementById("eqs-dark-theme-indicator").checkVisibility();
        loadBaseApp();
    });

    await loadBaseApp();
});
