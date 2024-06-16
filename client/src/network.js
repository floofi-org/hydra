async function fillNetwork() {
    let services = window.statusData['services'].filter(i => i.type === "network").sort((a, b) => {
        return (b.label === "Trusted Core Network") - (a.label === "Trusted Core Network");
    });
    await genericServiceFill(services, "network", false);
    document.getElementById("eqs-app-network").classList.add("eqs-container-show");
}
