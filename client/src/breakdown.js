function fillBreakdown() {
    let breakdownStartDay = new Date(Object.keys(statusData.breakdown)[0]).getTime() / 86400000;
    let breakdownEndDay = new Date(Object.keys(statusData.breakdown)[Object.keys(statusData.breakdown).length - 1]).getTime() / 86400000;
    let breakdown = [];

    for (let i = breakdownStartDay; i <= breakdownEndDay; i++) {
        let date = new Date(i * 86400000).toISOString().split("T")[0];
        breakdown.push(statusData.breakdown[date] ?? [100, 0, 0, 0]);
    }

    document.getElementById("sp-app-global-box-text-uptime").innerText = `${(Object.values(statusData['breakdown']).reduce((a, b) => a + b[0], 0) / Object.values(statusData['breakdown']).length).toFixed(3)}% uptime`;

    function buildSVG() {
        let width = document.getElementById("sp-app-global-box").clientWidth;
        breakdown.reverse();

        document.getElementById("sp-app-breakdown-box-inner").innerHTML = `
        <svg viewBox="-5 -5 ${width + 5} 155" class="chart" id="sp-app-breakdown-svg">
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
                    <stop offset="0" stop-color="var(--local-danger-reduced)" />
                    <stop offset="1" stop-color="var(--local-danger-transparent)" />
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
                stroke="var(--local-danger)"
                stroke-linejoin="round"
                stroke-width="3"
                points="${breakdown.map((i, j) => (j * (width / 90)) + "," + (i[0] - 3) * 1.5).join("\n")}"/>
            <polyline
                class="sp-redesign-hc-line"
                mask="url(#Mask1)"
                x="0"
                y="0"
                fill="none"
                stroke="var(--local-warning)"
                stroke-linejoin="round"
                stroke-width="3"
                points="${breakdown.map((i, j) => (j * (width / 90)) + "," + (i[0] - 3) * 1.5).join("\n")}"/>
            <polyline
                class="sp-redesign-hc-line"
                mask="url(#Mask2)"
                x="0"
                y="0"
                fill="none"
                stroke="var(--local-success)"
                stroke-linejoin="round"
                stroke-width="3"
                points="${breakdown.map((i, j) => (j * (width / 90)) + "," + (i[0] - 3) * 1.5).join("\n")}"/>
        </svg>
        <div id="sp-app-breakdown-box-cursor">
            <div id="sp-app-breakdown-box-cursor-line"></div>
            <div id="sp-app-breakdown-box-cursor-text">
                <span id="sp-app-breakdown-box-cursor-text-relative"></span><br>
                <span id="sp-app-breakdown-box-cursor-text-uptime"></span>
            </div>
        </div>
        `;

        breakdown.reverse();
        let inner = document.getElementById("sp-app-breakdown-box-inner");

        inner.onmouseleave = inner.ontouchend = (event) => {
            if (event.touches) {
                event.preventDefault();
                document.getElementById("sp-app-global").classList.remove("mobile-focus");
            }

            document.getElementById("sp-app-breakdown-box-cursor").style.opacity = "0";
            document.getElementById("sp-app-breakdown-box-cursor-text-relative").innerText = "";
            document.getElementById("sp-app-breakdown-box-cursor-text-uptime").innerText = "";
        }

        inner.onmousemove = inner.ontouchstart = inner.ontouchmove = (event) => {
            let realBreakdown = [...breakdown];

            while (realBreakdown.length < 90) {
                realBreakdown.unshift(null);
            }

            if (event.touches) {
                event.preventDefault();
                document.getElementById("sp-app-global").classList.add("mobile-focus");
            }

            let position = event.clientX ?? event.touches[0].clientX;
            let day = 90 - Math.round((((position - inner.getBoundingClientRect().left) / inner.clientWidth) * 90) - 0.7);

            if (!realBreakdown[90 - day]) {
                if (90 - day > 90) {
                    day = 0;
                    position = inner.getBoundingClientRect().right;
                } else {
                    day = 90;
                    position = inner.getBoundingClientRect().left;
                }
            }

            if (day < 16) {
                document.getElementById("sp-app-breakdown-box-cursor-text").classList.add("left");
            } else {
                document.getElementById("sp-app-breakdown-box-cursor-text").classList.remove("left");
            }

            if (realBreakdown[90 - day]) {
                document.getElementById("sp-app-breakdown-box-cursor").style.marginLeft = (position - inner.getBoundingClientRect().left) + "px";
                document.getElementById("sp-app-breakdown-box-cursor").style.opacity = "1";
            } else {
                document.getElementById("sp-app-breakdown-box-cursor").style.opacity = "0";
            }

            let date = document.getElementById("sp-app-breakdown-box-cursor-text-relative");

            if (day === 0) {
                date.innerText = "Today";
            } else if (day === 1) {
                date.innerText = "Yesterday";
            } else if (day < 7) {
                date.innerText = "Last " + new Date(new Date().getTime() - (86400000 * day)).toLocaleString("en-US", { weekday: "long" });
            } else {
                date.innerText = new Date(new Date().getTime() - (86400000 * day)).toLocaleString("en-IE", { weekday: "short", day: "numeric", month: "short" });
            }

            if (realBreakdown[90 - day]) {
                document.getElementById("sp-app-breakdown-box-cursor-text-uptime").innerText = realBreakdown[90 - day][0].toFixed(2) + "% uptime";
            }
        }
    }

    window.onresize = () => {
        document.getElementById("sp-app-breakdown-box-cursor").style.opacity = "0";
        buildSVG();
    }

    buildSVG();
}
