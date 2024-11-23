function error(e) {
    clearInterval(window.reloadInterval);
    console.error(e);

    document.getElementById("sp-app").innerHTML = document.getElementById("sp-template-error").innerHTML;
    document.getElementById("sp-app-error-reload").onclick = () => location.reload();
    document.getElementById("loader").style.display = "none";
}
