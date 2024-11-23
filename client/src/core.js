async function loadBaseApp() {
    window.oldDate = null;
    if (window.statusData) window.oldDate = statusData.time;

    try {
        window.statusData = await decodeStatusData();
    } catch (e) {
        error(e);
        return;
    }

    if (window.statusData.time === window.oldDate) return;

    try {
        document.getElementById("sp-app").innerHTML = "";
        document.getElementById("sp-app").innerHTML = document.getElementById("sp-template-app").innerHTML;

        document.getElementById("sp-app-copyright-text-year").innerText = " 2011-" + new Date().getFullYear();
        document.getElementById("sp-redesign-version").innerHTML = VERSION;
        completeLoad();

        await fillGlobal();

        fillUpdateTime();
        fillBreakdown();
        fillNetwork();
        fillWebsites();
        fillServers();
        completeLoading();
    } catch (e) {
        error(e);
    }
}

function completeLoading() {
    document.getElementById("sp-app-loader").style.display = "none";
    document.getElementById("sp-redesign-copyright").classList.add("sp-container-show");
}
