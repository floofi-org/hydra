function error(e) {
    console.error(e);

    document.getElementById("eqs-app").innerHTML = document.getElementById("eqs-template-error").innerHTML;
    document.getElementById("eqs-app-error-reload").onclick = () => location.reload();
    document.getElementById("eqs-loader").style.display = "none";
}
