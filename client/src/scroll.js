window.onscroll = () => {
    updateScroll();
}

function updateScroll() {
    if (window.scrollY === 0) {
        document.getElementById("sp-redesign-nav").classList.add("no-border");
    } else {
        document.getElementById("sp-redesign-nav").classList.remove("no-border");
    }
}
