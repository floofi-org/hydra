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
        document.getElementById("eqs-app").innerHTML = "";
        document.getElementById("eqs-app").innerHTML = document.getElementById("eqs-template-app").innerHTML;

        document.getElementById("eqs-app-nav-item-update-btn").onclick = () => location.reload();
        document.getElementById("eqs-app-copyright-text-year").innerText = " 2011-" + new Date().getFullYear();
        document.getElementById("eqs-app-version").innerHTML = "<span id='eqs-app-version-separator'> · </span>Version " + VERSION;
        document.getElementById("eqs-loader").style.display = "none";

        await fillGlobal();

        fillUpdateTime();
        fillBreakdown();
        fillNetwork();
        fillWebsites();
        completeLoading();
    } catch (e) {
        error(e);
    }
}

function completeLoading() {
    document.getElementById("eqs-app-loader").style.display = "none";
    document.getElementById("eqs-app-copyright").classList.add("eqs-container-show");
}
