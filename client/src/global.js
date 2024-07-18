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
            document.getElementById("eqs-app-global-box-text-main").innerHTML = "<div><p style='margin-top: 0; margin-bottom: 7px;'><b>" + window.statusData['notice']['title'] + "</b></p><p style='margin-top: 0; margin-bottom: 7px;'>" + window.statusData['notice']['description'] + "</p><a target='_blank' href='" + window.statusData['notice']['link'] +  "'>Read more.</a></div>";
            document.getElementById("eqs-app-global-box-icon").src = "./static/icons/status-info" + (hasDarkTheme ? "-dark" : "") + ".svg";
            document.getElementById("eqs-app-global-box-ping").innerText = "";
        } else {
            document.getElementById("eqs-app-global-box-ping").innerText = Math.round(statusData['ping']) + " ms";
            document.getElementById("eqs-app-global-box-text").classList.add("eqs-ellipsis");
            switch (window.statusData['global']) {
                case 0: // OK
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-success");
                    document.getElementById("eqs-app-global-box-text-main").innerText = "Everything is operational";
                    document.getElementById("eqs-app-global-box-icon").src = "./static/icons/status-success" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 1: // Warning
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-warning");
                    document.getElementById("eqs-app-global-box-text-main").innerText = down > 1 ? down + " services are not working properly" : "A service is not working properly";
                    document.getElementById("eqs-app-global-box-icon").src = "./static/icons/status-warning" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 2: // Down
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-critical");
                    document.getElementById("eqs-app-global-box-text-main").innerText = down > 1 ? down + " services are unavailable" : "A service is unavailable";
                    document.getElementById("eqs-app-global-box-icon").src = "./static/icons/status-critical" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
                case 3: // Maintenance
                    document.getElementById("eqs-app-global-box").classList.add("eqs-app-global-box-info");
                    document.getElementById("eqs-app-global-box-text-main").innerText = down > 1 ? down + " services are under maintenance" : "A service is under maintenance";
                    document.getElementById("eqs-app-global-box-icon").src = "./static/icons/status-info" + (hasDarkTheme ? "-dark" : "") + ".svg";
                    break;
            }
        }
    });
}
