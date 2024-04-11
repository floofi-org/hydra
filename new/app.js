const VERSION = "3.0.3";

function timeAgo(time, html) {
    if (!isNaN(parseInt(time))) {
        time = new Date(time).getTime();
    }

    let periods = ["second", "minute", "hour", "day", "week", "month", "year", "age"];

    let lengths = ["60", "60", "24", "7", "4.35", "12", "100"];

    let now = new Date().getTime();

    let difference = Math.round((now - time) / 1000);
    let tense;
    let period;

    if (difference <= 10 && difference >= 0) {
        return html ? `<span class="eqs-app-nav-desktop">now</span>` : "now";
    } else if (difference > 0) {
        tense = html ? `<span class="eqs-app-nav-desktop">ago</span>` : "ago";
    } else {
        tense = html ? `<span class="eqs-app-nav-desktop">later</span>` : "later";
    }

    let j;

    for (j = 0; difference >= lengths[j] && j < lengths.length - 1; j++) {
        difference /= lengths[j];
    }

    difference = Math.round(difference);

    period = periods[j];

    return `${difference} ${period}${difference > 1 ? "s" : ""} ${tense}`;
}

function error(e) {
    console.error(e);

    document.getElementById("eqs-app").innerHTML = `
<div id="eqs-app-error">
    <div id="eqs-app-error-inner">
        <img id="eqs-app-error-icon" src="./fatal.svg" alt="Fatal Error">
        <div id="eqs-app-error-title">Unable to load status page</div>
        <div id="eqs-app-error-message">An error has occurred while attempting to load the status page, please try again later.</div>
        <button id="eqs-app-error-reload" class="btn" tabindex="1">Try again</button>
        <div id="eqs-app-error-detail"></div>
    </div>
</div>
`;
    document.getElementById("eqs-app-error-reload").onclick = () => location.reload();
    document.getElementById("eqs-app-error-detail").innerText = e.message ?? e.name;
    document.getElementById("eqs-loader").style.display = "none";
    return;
}

async function loadBaseApp() {
    try {
        window.statusData = await (await fetch("https://raw.githubusercontent.com/equestriadev-status/realtime/master/public.json?_=" + ((crypto.randomUUID() + crypto.randomUUID() + crypto.randomUUID() + crypto.randomUUID()).replaceAll("-", "")), {cache: "no-store"})).json();
    } catch (e) {
        error(e);
        return;
    }

    try {
        document.getElementById("eqs-app").innerHTML = `
<nav id="eqs-app-nav">
    <div id="eqs-app-nav-inner" class="eqs-container">
        <img src="./logo.svg" alt="Equestria.dev" id="eqs-app-nav-logo">
        <div id="eqs-app-nav-items">
            <div id="eqs-app-nav-item-title">
                <span id="eqs-app-nav-item-inner" class="eqs-app-nav-desktop">Systems Status</span>
            </div>
            <div id="eqs-app-nav-item-update">
                <span id="eqs-app-nav-item-update-inner" class="eqs-ellipsis"></span>
                <button id="eqs-app-nav-item-update-btn">
                    <img alt="Refresh" src="./refresh.svg" id="eqs-app-nav-item-update-btn-icon">
                </button>
            </div>
        </div>
    </div>
</nav>

<main id="eqs-app-content">
    <div id="eqs-app-global" class="eqs-container eqs-container-lazy">
        <div id="eqs-app-global-box" class="eqs-box">
            <div id="eqs-app-global-box-icon-outer">
                <img alt="" id="eqs-app-global-box-icon" src="">
            </div>
            <div id="eqs-app-global-box-text-outer">
                <div id="eqs-app-global-box-text" class="eqs-ellipsis"></div>
                <div id="eqs-app-global-box-ping"></div>
            </div>
        </div>
    </div>
    
    <div id="eqs-app-breakdown" class="eqs-container eqs-container-lazy">
        <div id="eqs-app-breakdown-box" class="eqs-box">
            <div id="eqs-app-breakdown-box-inner"></div>
        </div>
    </div>
    
    <div id="eqs-app-network" class="eqs-container eqs-container-lazy">
        <div id="eqs-app-network-box" class="eqs-box">
            <div id="eqs-app-network-box-inner"></div>
        </div>
    </div>
    
    <div id="eqs-app-websites" class="eqs-container eqs-container-lazy">
        <div id="eqs-app-websites-box" class="eqs-box">
            <div id="eqs-app-websites-box-inner"></div>
        </div>
    </div>
    
    <div id="eqs-app-loader">
        <img id="eqs-loader-svg" src="./loader.svg" alt="Loading...">
    </div>
    
    <div id="eqs-app-copyright" class="eqs-container eqs-container-lazy">
        <img id="eqs-app-copyright-logo" src="./logo-footer.svg" alt="Equestria.dev">
        <div id="eqs-app-copyright-text">©<span id="eqs-app-copyright-text-year"></span> Equestria.dev Developers, All rights reserved.<span id="eqs-app-version"></span></div>
    </div>
</main>
`;

        document.getElementById("eqs-app-nav-item-update-btn").onclick = () => location.reload();
        document.getElementById("eqs-app-copyright-text-year").innerText = " 2011-" + new Date().getFullYear();
        document.getElementById("eqs-app-version").innerText = " · Version " + VERSION;
        document.getElementById("eqs-loader").style.display = "none";

        await fillUpdateTime();
        await fillGlobal();
        await fillBreakdown();
        await fillNetwork();
        await fillWebsites();
        await completeLoading();
    } catch (e) {
        error(e);
    }
}

function completeLoading() {
    document.getElementById("eqs-app-loader").style.display = "none";
    document.getElementById("eqs-app-copyright").classList.add("eqs-container-show");
}

function fillUpdateTime() {
    document.getElementById("eqs-app-nav-item-update-inner").innerHTML = "<span class='eqs-app-nav-desktop'>Last updated </span>" + timeAgo(window.statusData['time'], true);
}

function fillGlobal() {
    return new Promise((res) => {
        let down = statusData['services'].filter(i => i.status > 0).length;

        document.getElementById("eqs-app-global-box-icon").onload = () => {
            document.getElementById("eqs-app-global").classList.add("eqs-container-show");
            res();
        }

        document.getElementById("eqs-app-global-box-ping").innerText = Math.round(statusData['ping']) + " ms";

        switch (window.statusData['global']) {
            case 0: // OK
                document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-success");
                document.getElementById("eqs-app-global-box-text").innerText = "Everything is operational";
                document.getElementById("eqs-app-global-box-icon").src = "./status-success.svg";
                break;
            case 1: // Warning
                document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-warning");
                document.getElementById("eqs-app-global-box-text").innerText = down > 1 ? down + " services are slower than usual" : "A service is slower than usual";
                document.getElementById("eqs-app-global-box-icon").src = "./status-warning.svg";
                break;
            case 2: // Down
                document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-critical");
                document.getElementById("eqs-app-global-box-text").innerText = down > 1 ? down + " services are unavailable" : "A service is unavailable";
                document.getElementById("eqs-app-global-box-icon").src = "./status-critical.svg";
                break;
            case 3: // Maintenance
                document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-info");
                document.getElementById("eqs-app-global-box-text").innerText = down > 1 ? down + " services are under maintenance" : "A service is under maintenance";
                document.getElementById("eqs-app-global-box-icon").src = "./status-info.svg";
                break;
        }
    });
}

function fillBreakdown() {
    let breakdown = Object.entries(window.statusData['breakdown']).sort((a, b) => new Date(b[0]) - new Date(a[0]));

    for (let i = 0; i < 7; i++) {
        for (let j = 0; j < 7; j++) {
            let percentages = [0, 0, 0, 0];
            let item = breakdown[(i * 7) + j][1];
            percentages[0] = item[0];
            percentages[1] = item[0] + item[1];
            percentages[2] = item[0] + item[1] + item[2];
            percentages[3] = item[0] + item[1] + item[2] + item[3];

            document.getElementById("eqs-app-breakdown-box-inner").insertAdjacentHTML("beforeend", `
            <div class="eqs-app-breakdown-item" id="eqs-app-breakdown-item-${(i * 7) + j}" style="opacity: ${100 - (((i * 7) + j) / 49) * 100}%; background-image: linear-gradient(90deg,
                var(--eqs-app-breakdown-success) 0%, var(--eqs-app-breakdown-success) ${percentages[0]}%,
                var(--eqs-app-breakdown-warning) ${percentages[0] + 0.001}%, var(--eqs-app-breakdown-warning) ${percentages[1]}%,
                var(--eqs-app-breakdown-critical) ${percentages[1] + 0.001}%, var(--eqs-app-breakdown-critical) ${percentages[2]}%,
                var(--eqs-app-breakdown-info) ${percentages[2] + 0.001}%, var(--eqs-app-breakdown-info) ${percentages[3]}%
            );"></div>
            `);
        }
    }

    document.getElementById("eqs-app-breakdown").classList.add("eqs-container-show");
}

function genericServiceFill(list, name, link) {
    for (let service of list) {
        document.getElementById("eqs-app-" + name + "-box-inner").insertAdjacentHTML("beforeend", `
        <div id="eqs-app-service-${service.id}" class="eqs-app-service eqs-app-${name}-service">
            <div id="eqs-app-service-${service.id}-label" class="eqs-ellipsis eqs-app-service-label">${link ? `<a id="eqs-app-service-${service.id}-link" class="eqs-app-service-link" target="_blank" href="https://${service.label}/">${service.label}</a>` : service.label}</div>
            <div id="eqs-app-service-${service.id}-status" class="eqs-app-service-status eqs-app-service-status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}">
                <img id="eqs-app-service-${service.id}-status-icon" class="eqs-app-service-status-icon" alt="" src="./status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}.svg">
                <span id="eqs-app-service-${service.id}-status-message" class="eqs-ellipsis eqs-app-service-status-message">${service.status <= 1 ? service.ping + " ms" : (service.status === 2 ? 'Offline' : 'Maintenance')}</span>
            </div>
        </div>
        `);
    }
}

async function fillNetwork() {
    let services = window.statusData['services'].filter(i => i.type === "network");
    await genericServiceFill(services, "network", false);
    document.getElementById("eqs-app-network").classList.add("eqs-container-show");
}

async function fillWebsites() {
    let services = window.statusData['services'].filter(i => i.type === "websites");
    await genericServiceFill(services, "websites", true);
    document.getElementById("eqs-app-websites").classList.add("eqs-container-show");
}

// -----

window.addEventListener('load', async () => {
    await loadBaseApp();
});
