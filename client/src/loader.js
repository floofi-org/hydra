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
        "./static/logo/favicon.png", "./static/logo/footer.png",
        "./static/logo/powered-by-vercel.svg", "./static/logo/logo.png",
        "./static/icons/status-critical-dark.svg",
        "./static/icons/status-info-dark.svg",
        "./static/icons/status-success-dark.svg",
        "./static/icons/status-warning-dark.svg",
        "./static/hosting/self.webp", "./static/hosting/gitbook.webp",
        "./static/hosting/ovh.webp", "./static/hosting/scaleway.webp",
        "./static/hosting/vercel.webp", "./static/hosting/azure.webp"
    ]) {
        await preloadImage(i);
    }

    window.hasDarkTheme = true;

    await loadBaseApp();

    window.reloadInterval = setInterval(async () => {
        if (!window.statusData) return;
        fillUpdateTime();

        if (new Date().getTime() - new Date(window.statusData['time']).getTime() >= 600000) {
            await loadBaseApp();
        }
    }, 1000);
});
