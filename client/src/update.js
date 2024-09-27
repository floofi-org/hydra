function timeAgo(time, html) {
    if (!isNaN(parseInt(time))) time = new Date(time).getTime();

    let periods = ["second", "minute", "hour", "day", "week", "month", "year", "age"];
    let lengths = ["60", "60", "24", "7", "4.35", "12", "100"];
    let now = new Date().getTime();

    let difference = Math.round((now - time) / 1000);
    let tense;
    let period;

    if (difference <= 10 && difference >= 0) {
        return html ? `<span class="sp-app-nav-desktop">now</span>` : "now";
    } else if (difference > 0) {
        tense = html ? `<span class="sp-app-nav-desktop">ago</span>` : "ago";
    } else {
        tense = html ? `<span class="sp-app-nav-desktop">later</span>` : "later";
    }

    let j;

    for (j = 0; difference >= lengths[j] && j < lengths.length - 1; j++) {
        difference /= lengths[j];
    }

    difference = Math.round(difference);
    period = periods[j];
    return `${difference} ${period}${difference > 1 ? "s" : ""} ${tense}`;
}

let fillUpdateTime = () => document.getElementById("sp-redesign-nav-update").innerHTML = "<span class='sp-app-nav-desktop'>Last updated </span>" + timeAgo(window.statusData['time'], true);
