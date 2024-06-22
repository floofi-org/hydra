async function loadBaseApp() {
    window.oldDate = null;
    if (window.statusData) window.oldDate = statusData.time;

    try {
        window.statusData = await (await fetch(URL)).json();
    } catch (e) {
        error(e);
        return;
    }

    if (window.statusData.time === window.oldDate) return;

    try {
        document.getElementById("eqs-app").innerHTML = "";
        document.getElementById("eqs-app").innerHTML = `
<nav id="eqs-app-nav">
    <div id="eqs-app-nav-inner" class="eqs-container">
        <img src="../static/logo/logo.svg" alt="Equestria.dev" id="eqs-app-nav-logo">
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
                <div id="eqs-app-global-box-text" class="eqs-ellipsis">
                    <span id="eqs-app-global-box-text-main"></span>  
                    <span id="eqs-app-global-box-text-uptime">${(Object.values(statusData['breakdown']).reduce((a, b) => a + b[0], 0) / Object.values(statusData['breakdown']).length).toFixed(3)}% uptime</span>              
                </div>
                <div id="eqs-app-global-box-ping"></div>
            </div>
        </div>
        <div id="eqs-app-breakdown-box" class="eqs-box">
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
        <img id="eqs-app-copyright-logo-dark" src="../static/logo/logo-footer-dark.svg" alt="Equestria.dev">
        <img id="eqs-app-copyright-logo-light" src="../static/logo/logo-footer-light.svg" alt="Equestria.dev">
        <div id="eqs-app-copyright-text">©<span id="eqs-app-copyright-text-year"></span> Equestria.dev Developers, All rights reserved.<span id="eqs-app-version"></span></div>
        <div id="eqs-app-copyright-powered-by">
            <a href="https://vercel.com/?utm_source=powered-by-vercel" target="_blank">
                <img alt="Powered by Vercel" src="../static/logo/powered-by-vercel.svg" id="eqs-app-copyright-powered-by-img">
            </a>
        </div>
    </div>
</main>
`;

        document.getElementById("eqs-app-nav-item-update-btn").onclick = () => location.reload();
        document.getElementById("eqs-app-copyright-text-year").innerText = " 2011-" + new Date().getFullYear();
        document.getElementById("eqs-app-version").innerHTML = "<span id='eqs-app-version-separator'> · </span>Version " + VERSION;
        document.getElementById("eqs-loader").style.display = "none";

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
    document.getElementById("eqs-app-loader").style.display = "none";
    document.getElementById("eqs-app-copyright").classList.add("eqs-container-show");
}
