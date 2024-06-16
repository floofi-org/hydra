async function fillWebsites() {
    let services = window.statusData['services'].filter(i => i.type === "websites");
    await genericServiceFill(services, "websites", true);
    document.getElementById("eqs-app-websites").classList.add("eqs-container-show");
}
