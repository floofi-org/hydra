const VERSION = "3.3.1";
const URL = "https://d6gd1hq6b89h1s1v.public.blob.vercel-storage.com/public/status.json";

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
        <svg id="eqs-app-error-icon" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="6.5" stroke="var(--theme-icon-stroke)"/>
            <path d="M8 4.5L8 8" stroke="var(--theme-icon-stroke)" stroke-width="1.2" stroke-linecap="round"/>
            <circle cx="8" cy="10.75" r="0.75" fill="var(--theme-icon-stroke)"/>
        </svg>
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
}

async function loadBaseApp() {
    document.getElementById("eqs-app").innerHTML = "";

    try {
        window.statusData = await (await fetch(URL)).json();
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
                    <svg id="eqs-app-nav-item-update-btn-icon" width="16" height="16" viewBox="0 0 16 16" fill="none">
                        <path d="M2.5 9V8C2.5 4.96243 4.96243 2.5 8 2.5C9.10679 2.5 10.1372 2.82692 11 3.38947" stroke="var(--theme-icon-stroke)" stroke-linecap="round"/>
                        <path d="M5 12.6105C5.86278 13.1731 6.89321 13.5 8 13.5C11.0376 13.5 13.5 11.0376 13.5 8V7" stroke="var(--theme-icon-stroke)" stroke-linecap="round"/>
                        <path d="M0.49997 7.50027L2.5 9.5L4.49998 7.50023" stroke="var(--theme-icon-stroke)" stroke-linecap="round"/>
                        <path d="M11.5 8.49982L13.5 6.5L15.5 8.49982" stroke="var(--theme-icon-stroke)" stroke-linecap="round"/>
                    </svg>
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
            <h2 class="eqs-section-title" id="eqs-app-breakdown-title">Overview</h2>
            <div id="eqs-app-breakdown-box-inner"></div>
        </div>
    </div>
    
    <div id="eqs-app-network" class="eqs-container eqs-container-lazy">
        <div id="eqs-app-network-box" class="eqs-box">
            <h2 class="eqs-section-title" id="eqs-app-network-title">Network</h2>
            <div id="eqs-app-network-box-inner"></div>
        </div>
    </div>
    
    <div id="eqs-app-websites" class="eqs-container eqs-container-lazy">
        <div id="eqs-app-websites-box" class="eqs-box">
            <h2 class="eqs-section-title" id="eqs-app-websites-title">Websites</h2>
            <div id="eqs-app-websites-box-inner"></div>
        </div>
    </div>
    
    <div id="eqs-app-servers" class="eqs-container eqs-container-lazy">
        <div id="eqs-app-servers-box" class="eqs-box">
            <h2 class="eqs-section-title" id="eqs-app-servers-title">Servers</h2>
            <div id="eqs-app-servers-box-inner"></div>
        </div>
    </div>
    
    <div id="eqs-app-loader">
        <svg id="eqs-loader-svg" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid" style="shape-rendering: auto; display: block; background: transparent;">
            <g transform="rotate(0 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.875s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
                <g transform="rotate(45 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.75s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
                <g transform="rotate(90 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.625s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
                <g transform="rotate(135 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.5s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
                <g transform="rotate(180 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.375s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
                <g transform="rotate(225 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.25s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
                <g transform="rotate(270 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.125s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
                <g transform="rotate(315 50 50)">
                    <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                        <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="0s"
                                 repeatCount="indefinite"/>
                    </rect>
                </g>
        </svg>
    </div>
    
    <div id="eqs-app-copyright" class="eqs-container eqs-container-lazy">
        <img id="eqs-app-copyright-logo-dark" src="logo-footer-dark.svg" alt="Equestria.dev">
        <img id="eqs-app-copyright-logo-light" src="logo-footer-light.svg" alt="Equestria.dev">
        <div id="eqs-app-copyright-text">©<span id="eqs-app-copyright-text-year"></span> Equestria.dev Developers, All rights reserved.<span id="eqs-app-version"></span></div>
        <div id="eqs-app-copyright-powered-by">
            <a href="https://vercel.com/?utm_source=powered-by-vercel" target="_blank">
                <img alt="Powered by Vercel" src="./powered-by-vercel.svg" id="eqs-app-copyright-powered-by-img">
            </a>
        </div>
    </div>
</main>
`;

        document.getElementById("eqs-app-nav-item-update-btn").onclick = () => location.reload();
        document.getElementById("eqs-app-copyright-text-year").innerText = " 2011-" + new Date().getFullYear();
        document.getElementById("eqs-app-version").innerHTML = "<span id='eqs-app-version-separator'> · </span>Version " + VERSION;
        document.getElementById("eqs-loader").style.display = "none";

        await fillUpdateTime();
        await fillGlobal();
        await fillBreakdown();
        await fillNetwork();
        await fillWebsites();
        await fillServers();
        await completeLoading();
    } catch (e) {
        error(e);
    }
}

function completeLoading() {
    document.getElementById("eqs-app-loader").style.display = "none";
    document.getElementById("eqs-app-copyright").classList.add("eqs-container-show");

    setInterval(async () => {
        await fillUpdateTime();

        if (new Date().getTime() - new Date(window.statusData['time']).getTime() >= 600000) {
            await loadBaseApp();
        }
    }, 1000);
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

        if (window.statusData['notice']) {
            document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-info");
            document.getElementById("eqs-app-global-box-text").classList.remove("eqs-ellipsis");
            document.getElementById("eqs-app-global-box-text").innerHTML = "<div><p style='margin-top: 0; margin-bottom: 7px;'><b>" + window.statusData['notice']['title'] + "</b></p><p style='margin-top: 0; margin-bottom: 7px;'>" + window.statusData['notice']['description'] + "</p><a target='_blank' href='" + window.statusData['notice']['link'] +  "'>Read more.</a></div>";
            document.getElementById("eqs-app-global-box-icon").src = "./status-info" + (hasDarkTheme ? "-dark" : "") + ".svg";
            document.getElementById("eqs-app-global-box-ping").innerText = "";
        } else {
            document.getElementById("eqs-app-global-box-ping").innerText = Math.round(statusData['ping']) + " ms";
            document.getElementById("eqs-app-global-box-text").classList.add("eqs-ellipsis");
            switch (window.statusData['global']) {
                case 0: // OK
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-success");
                    document.getElementById("eqs-app-global-box-text").innerText = "Everything is operational";
                    document.getElementById("eqs-app-global-box-icon").src = "./status-success" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 1: // Warning
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-warning");
                    document.getElementById("eqs-app-global-box-text").innerText = down > 1 ? down + " services are slower than usual" : "A service is slower than usual";
                    document.getElementById("eqs-app-global-box-icon").src = "./status-warning" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 2: // Down
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-critical");
                    document.getElementById("eqs-app-global-box-text").innerText = down > 1 ? down + " services are unavailable" : "A service is unavailable";
                    document.getElementById("eqs-app-global-box-icon").src = "./status-critical" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 3: // Maintenance
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-info");
                    document.getElementById("eqs-app-global-box-text").innerText = down > 1 ? down + " services are under maintenance" : "A service is under maintenance";
                    document.getElementById("eqs-app-global-box-icon").src = "./status-info" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
            }
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
        if (service.ping >= 5000) {
            service.status = 2;
        }

        document.getElementById("eqs-app-" + name + "-box-inner").insertAdjacentHTML("beforeend", `
        <div id="eqs-app-service-${service.id}" class="eqs-app-service eqs-app-${name}-service">
            <div id="eqs-app-service-${service.id}-label" class="eqs-ellipsis eqs-app-service-label">${link ? `<a id="eqs-app-service-${service.id}-link" class="eqs-app-service-link" target="_blank" href="https://${service.label}/">${service.label}</a>` : service.label}</div>
            <div id="eqs-app-service-${service.id}-status" class="eqs-app-service-status eqs-app-service-status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}">
                <img id="eqs-app-service-${service.id}-status-icon" class="eqs-app-service-status-icon" alt="" src="./status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}${hasDarkTheme ? "-dark" : ""}.svg">
                <span id="eqs-app-service-${service.id}-status-message" class="eqs-ellipsis eqs-app-service-status-message">${service.status <= 1 ? service.ping + " ms" : (service.status === 2 ? 'Offline' : 'Maintenance')}</span>
            </div>
            <div id="eqs-app-service-${service.id}-hosting" class="eqs-app-service-hosting">
                <img alt="${service['hosting']}" src="./hosting/${service['hosting']}.png" id="eqs-app-service-${service.id}-hosting-img" class="eqs-app-service-hosting-img">
            </div>
        </div>
        `);
    }
}

async function fillNetwork() {
    let services = window.statusData['services'].filter(i => i.type === "network").sort((a, b) => {
        return (b.label === "Trusted Core Network") - (a.label === "Trusted Core Network");
    });
    await genericServiceFill(services, "network", false);
    document.getElementById("eqs-app-network").classList.add("eqs-container-show");
}

async function fillWebsites() {
    let services = window.statusData['services'].filter(i => i.type === "websites");
    await genericServiceFill(services, "websites", true);
    document.getElementById("eqs-app-websites").classList.add("eqs-container-show");
}

async function fillServers() {
    let services = window.statusData['services'].filter(i => i.type === "servers");
    await genericServiceFill(services, "servers", false);
    document.getElementById("eqs-app-servers").classList.add("eqs-container-show");
}

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

// -----

window.addEventListener('load', async () => {
    await preloadImage("./logo.svg");
    await preloadImage("./logo-footer-dark.svg");
    await preloadImage("./logo-footer-light.svg");
    await preloadImage("./status-critical.svg");
    await preloadImage("./status-critical-dark.svg");
    await preloadImage("./status-info.svg");
    await preloadImage("./status-info-dark.svg");
    await preloadImage("./status-success.svg");
    await preloadImage("./status-success-dark.svg");
    await preloadImage("./status-warning.svg");
    await preloadImage("./status-warning-dark.svg");

    window.hasDarkTheme = document.getElementById("eqs-dark-theme-indicator").checkVisibility();

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
        window.hasDarkTheme = document.getElementById("eqs-dark-theme-indicator").checkVisibility();
        loadBaseApp();
    });

    await loadBaseApp();
});
