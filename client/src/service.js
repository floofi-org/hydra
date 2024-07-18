function genericServiceFill(name, link) {
    let list = window.statusData['services'].filter(i => i.type === name);

    for (let service of list) {
        if (service.ping >= 5000) service.status = 2;

        document.getElementById("eqs-app-" + name + "-box-inner").insertAdjacentHTML("beforeend", `
        <div id="eqs-app-service-${service.id}" class="eqs-app-service eqs-app-${name}-service">
            <div id="eqs-app-service-${service.id}-label" class="eqs-ellipsis eqs-app-service-label">${link ? `<a id="eqs-app-service-${service.id}-link" class="eqs-app-service-link" target="_blank" href="https://${service.label}/">${service.label}</a>` : service.label}</div>
            <div id="eqs-app-service-${service.id}-status" class="eqs-app-service-status eqs-app-service-status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}">
                <img id="eqs-app-service-${service.id}-status-icon" class="eqs-app-service-status-icon" alt="" src="./static/icons/status-${service.status === 0 ? 'success' : (service.status === 1 ? 'warning' : (service.status === 2 ? 'critical' : 'info'))}${hasDarkTheme ? "-dark" : ""}.svg">
                <span id="eqs-app-service-${service.id}-status-message" class="eqs-ellipsis eqs-app-service-status-message">${service.status <= 1 ? service.ping + " ms" : (service.status === 2 ? 'Offline' : 'Maintenance')}</span>
            </div>
            <div id="eqs-app-service-${service.id}-hosting" class="eqs-app-service-hosting">
                <img alt="${service['hosting']}" src="./static/hosting/${service['hosting']}.png" id="eqs-app-service-${service.id}-hosting-img" class="eqs-app-service-hosting-img">
            </div>
        </div>
        `);
    }

    document.getElementById("eqs-app-" + name).classList.add("eqs-container-show");
}

let fillWebsites = () => genericServiceFill("websites", true);
let fillServers = () => genericServiceFill("servers", true);
let fillNetwork = () => genericServiceFill("network", true);
