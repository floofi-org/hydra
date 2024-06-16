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
    for (let i of [
        "./static/logo/logo.svg", "./static/logo/logo-footer-dark.svg",
        "./static/logo/logo-footer-light.svg", "./static/logo/powered-by-vercel.svg",
        "./static/icons/status-critical.svg", "./static/icons/status-critical-dark.svg",
        "./static/icons/status-info.svg", "./static/icons/status-info-dark.svg",
        "./static/icons/status-success.svg", "./static/icons/status-success-dark.svg",
        "./static/icons/status-warning.svg", "./static/icons/status-warning-dark.svg",
        "./static/hosting/equestriadev.png", "./static/hosting/gitbook.png",
        "./static/hosting/ovh.png", "./static/hosting/scaleway.png",
        "./static/hosting/vercel.png"
    ]) {
        await preloadImage(i);
    }

    window.hasDarkTheme = document.getElementById("eqs-dark-theme-indicator").checkVisibility();

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        window.hasDarkTheme = document.getElementById("eqs-dark-theme-indicator").checkVisibility();
        loadBaseApp();
    });

    await loadBaseApp();
});
