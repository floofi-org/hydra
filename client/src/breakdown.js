function fillBreakdown() {
    document.getElementById("eqs-app-global-box-text-uptime").innerText = `${(Object.values(statusData['breakdown']).reduce((a, b) => a + b[0], 0) / Object.values(statusData['breakdown']).length).toFixed(3)}% uptime`;
    let breakdown = Object.values(window.statusData['breakdown']).reverse();

    function buildSVG() {
        let width = document.getElementById("eqs-app-global-box").clientWidth;

        document.getElementById("eqs-app-breakdown-box-inner").innerHTML = `
        <svg viewBox="-5 -5 ${width + 5} 155" class="chart" id="eqs-app-breakdown-svg">
            <defs>
                <linearGradient id="Gradient1" gradientTransform="rotate(90)">
                    <stop offset="0.75" stop-color="black" />
                    <stop offset="1" stop-color="white" />
                </linearGradient>
                <linearGradient id="Gradient2" gradientTransform="rotate(90)">
                    <stop offset="0.5" stop-color="black" />
                    <stop offset="1" stop-color="white" />
                </linearGradient>
                <linearGradient id="Gradient3" gradientTransform="rotate(90)">
                    <stop offset="0" stop-color="#db3b4b22" />
                    <stop offset="1" stop-color="#db3b4b00" />
                </linearGradient>
                <linearGradient id="Gradient4">
                    <stop offset="0.75" stop-color="black" />
                    <stop offset="1" stop-color="white" />
                </linearGradient>
                <mask id="Mask1">
                    <rect x="0" y="0" width="${width + 5}" height="155" fill="url(#Gradient1)" />
                </mask>
                <mask id="Mask2">
                    <rect x="0" y="0" width="${width + 5}" height="155" fill="url(#Gradient2)" />
                </mask>
                <mask id="Mask3">
                    <rect x="0" y="0" width="${width + 5}" height="155" fill="url(#Gradient3)" />
                </mask>
                <mask id="Mask4">
                    <rect x="0" y="0" width="${width + 5}" height="155" fill="url(#Gradient4)" />
                </mask>
            </defs>
            <polyline
                x="0"
                y="0"
                fill="url(#Gradient3)"
                stroke="var(--theme-red)"
                stroke-linejoin="round"
                stroke-width="3"
                points="${breakdown.map((i, j) => (j * (width / 90)) + "," + (i[0] - 3) * 1.5).join("\n")}"/>
            <polyline
                mask="url(#Mask1)"
                x="0"
                y="0"
                fill="none"
                stroke="var(--theme-yellow)"
                stroke-linejoin="round"
                stroke-width="3"
                points="${breakdown.map((i, j) => (j * (width / 90)) + "," + (i[0] - 3) * 1.5).join("\n")}"/>
            <polyline
                mask="url(#Mask2)"
                x="0"
                y="0"
                fill="none"
                stroke="var(--theme-green)"
                stroke-linejoin="round"
                stroke-width="3"
                points="${breakdown.map((i, j) => (j * (width / 90)) + "," + (i[0] - 3) * 1.5).join("\n")}"/>
            <rect x="0" y="0" width="${width + 5}" height="155" fill="var(--theme-secondary-bg)" mask="url(#Mask4)" />
        </svg>
        <div id="eqs-app-breakdown-box-cursor">
            <div id="eqs-app-breakdown-box-cursor-line"></div>
            <div id="eqs-app-breakdown-box-cursor-text">
                <span id="eqs-app-breakdown-box-cursor-text-relative"></span><br>
                <span id="eqs-app-breakdown-box-cursor-text-uptime"></span>
            </div>
        </div>
        `;

        document.getElementById("eqs-app-breakdown-box-inner").onmouseenter = () => {
            document.getElementById("eqs-app-breakdown-box-cursor").style.opacity = "1";
        }

        document.getElementById("eqs-app-breakdown-box-inner").onmouseleave = document.body.ontouchend = () => {
            document.getElementById("eqs-app-breakdown-box-cursor").style.opacity = "0";
            document.getElementById("eqs-app-breakdown-box-cursor-text-relative").innerText = "";
            document.getElementById("eqs-app-breakdown-box-cursor-text-uptime").innerText = "";
        }

        document.getElementById("eqs-app-breakdown-box-inner").onmousemove = (event) => {
            let day = Math.round(((event.clientX - document.getElementById("eqs-app-breakdown-box-inner").getBoundingClientRect().left) / document.getElementById("eqs-app-breakdown-box-inner").clientWidth) * 90);

            if (day > 75) {
                document.getElementById("eqs-app-breakdown-box-cursor").style.opacity = "0";
                document.getElementById("eqs-app-breakdown-box-cursor-text-relative").innerText = "";
                document.getElementById("eqs-app-breakdown-box-cursor-text-uptime").innerText = "";
                return;
            } else {
                document.getElementById("eqs-app-breakdown-box-cursor").style.opacity = "1";
            }

            document.getElementById("eqs-app-breakdown-box-cursor").style.marginLeft = (3 + document.getElementById("eqs-app-breakdown-box-inner").clientWidth * (day / 90)) + "px";

            if (day === 0) {
                document.getElementById("eqs-app-breakdown-box-cursor-text-relative").innerText = "Today";
            } else if (day === 1) {
                document.getElementById("eqs-app-breakdown-box-cursor-text-relative").innerText = "Yesterday";
            } else if (day < 7) {
                document.getElementById("eqs-app-breakdown-box-cursor-text-relative").innerText = "Last " + new Date(new Date().getTime() - (86400000 * day)).toLocaleString("en-US", { weekday: "long" });
            } else {
                document.getElementById("eqs-app-breakdown-box-cursor-text-relative").innerText = new Date(new Date().getTime() - (86400000 * day)).toLocaleString("en-IE", { weekday: "short", day: "numeric", month: "short" });
            }

            document.getElementById("eqs-app-breakdown-box-cursor-text-uptime").innerText = breakdown[day][0].toFixed(2) + "% uptime";
        }
    }

    window.onresize = () => {
        document.getElementById("eqs-app-breakdown-box-cursor").style.opacity = "0";
        buildSVG();
    }

    buildSVG();
}
