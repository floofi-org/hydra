function error(e) {
    console.error(e);

    document.getElementById("eqs-app").innerHTML = `
<div id="eqs-app-error">
    <div id="eqs-app-error-inner">
        <svg id="eqs-app-error-icon" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="6.5" stroke="var(--theme-icon-stroke)"/>
            <path d="M8 4.5L8 8" stroke="var(--theme-icon-stroke)" stroke-width="1.2" stroke-linecap="round"/>
            <circle cx="8" cy="10.75" r="0.75" fill="var(--theme-icon-stroke)"/>
        </svg>
        <div id="eqs-app-error-title">Unable to load status page</div>
        <div id="eqs-app-error-message">An error has occurred while attempting to load the status page, please try again later.</div>
        <button id="eqs-app-error-reload" class="btn" tabindex="1">Try again</button>
        <div id="eqs-app-error-detail"></div>
    </div>
</div>
`;
    document.getElementById("eqs-app-error-reload").onclick = () => location.reload();
    document.getElementById("eqs-app-error-detail").innerText = e.message ?? e.name;
    document.getElementById("eqs-loader").style.display = "none";
}
