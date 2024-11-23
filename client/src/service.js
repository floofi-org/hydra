function genericServiceFill(name, link) {
    let list = window.statusData['services'].filter(i => i.type === name);

    for (let service of list) {
        if (service.ping >= 5000) service.status = 2;

        document.getElementById("sp-app-" + name + "-box-inner").insertAdjacentHTML("beforeend", `
        <a id="sp-app-service-${service.id}" class="fella-list-item fella-list-item-lg sp-app-service sp-app-${name}-service ${link ? 'fella-list-link' : ''}" target="_blank" ${link ? `href="https://${service.label}/"${service.label}` : ''}>
            <img id="sp-app-service-${service.id}-status-icon" class="sp-app-service-status-icon" alt="" src="./static/icons/status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}${hasDarkTheme ? "-dark" : ""}.svg">
            <div id="sp-app-service-${service.id}-label" class="sp-ellipsis sp-app-service-label">${service.label}</div>
            <div id="sp-app-service-${service.id}-hosting" class="sp-app-service-hosting fella-badge">
                <img alt="${service['hosting']}" src="./static/hosting/${service['hosting']}.webp" id="sp-app-service-${service.id}-hosting-img" class="sp-redesign-hosting-logo fella-badge-icon">
                <span class="sp-redesign-hosting-label fella-badge-label">${HOSTING_PROVIDER_LABELS[service['hosting']] ?? service['hosting']}</span>
            </div>
            <div id="sp-app-service-${service.id}-status" class="sp-app-service-status sp-app-service-status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}">
                <span id="sp-app-service-${service.id}-status-message" class="sp-ellipsis sp-app-service-status-message">${service.status <= 1 ? service.ping + " ms" : (service.status === 2 ? 'Offline' : 'Maintenance')}</span>
            </div>
        </a>
        `);
    }

    document.getElementById("sp-app-" + name).classList.add("sp-container-show");
}

let fillWebsites = () => genericServiceFill("websites", true);
let fillNetwork = () => genericServiceFill("network", false);
let fillServers = () => genericServiceFill("servers", false);
