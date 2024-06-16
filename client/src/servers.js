async function fillServers() {
    let services = window.statusData['services'].filter(i => i.type === "servers");
    await genericServiceFill(services, "servers", false);
    document.getElementById("eqs-app-servers").classList.add("eqs-container-show");
}
