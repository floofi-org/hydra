function fillGlobal() {
    return new Promise((res) => {
        let down = statusData['services'].filter(i => i.status > 0).length;

        document.getElementById("sp-app-global-box-icon").onload = () => {
            document.getElementById("sp-app-global").classList.add("sp-container-show");
            res();
        }

        if (window.statusData['notice']) {
            document.getElementById("sp-app-global").classList.add("sp-app-global-box-info");
            document.getElementById("sp-app-global-box-text").classList.remove("sp-ellipsis");
            document.getElementById("sp-app-global-box-text-main").innerHTML = "<div><p style='margin-top: 0; margin-bottom: 7px;'><b>" + window.statusData['notice']['title'] + "</b></p><p style='margin-top: 0; margin-bottom: 7px;'>" + window.statusData['notice']['description'] + "</p>" + (window.statusData['notice']['link'] && window.statusData['notice']['link'] !== "null" && window.statusData['notice']['link'] !== null ? "<a target='_blank' href='" + window.statusData['notice']['link'] +  "'>Read more.</a>" : "") + "</div>";
            document.getElementById("sp-app-global-box-icon").src = "./static/icons/status-info" + (hasDarkTheme ? "-dark" : "") + ".svg";
            document.getElementById("sp-app-global-box-ping").innerText = "";
        } else {
            document.getElementById("sp-app-global-box-ping").innerText = Math.round(statusData['ping']) + " ms";
            document.getElementById("sp-app-global-box-text").classList.add("sp-ellipsis");
            switch (window.statusData['global']) {
                case 0: // OK
                    document.getElementById("sp-app-global").classList.add("sp-app-global-box-success");
                    document.getElementById("sp-app-global-box-text-main").innerText = "All systems normal.";
                    document.getElementById("sp-app-global-box-icon").src = "./static/icons/status-success" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 1: // Warning
                    document.getElementById("sp-app-global").classList.add("sp-app-global-box-warning");
                    document.getElementById("sp-app-global-box-text-main").innerText = "Temporary alert.";
                    document.getElementById("sp-app-global-box-icon").src = "./static/icons/status-warning" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 2: // Down
                    document.getElementById("sp-app-global").classList.add("sp-app-global-box-critical");
                    document.getElementById("sp-app-global-box-text-main").innerText = "Temporary outage.";
                    document.getElementById("sp-app-global-box-icon").src = "./static/icons/status-critical" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 3: // Maintenance
                    document.getElementById("sp-app-global").classList.add("sp-app-global-box-info");
                    document.getElementById("sp-app-global-box-text-main").innerText = "Down for maintenance.";
                    document.getElementById("sp-app-global-box-icon").src = "./static/icons/status-info" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
            }
        }
    });
}
