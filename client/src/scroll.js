window.onscroll = () => {
    updateScroll();
}

function updateScroll() {
    if (window.scrollY === 0) {
        document.getElementById("sp-redesign-nav").classList.add("fella-nav-no-border");
    } else {
        document.getElementById("sp-redesign-nav").classList.remove("fella-nav-no-border");
    }
}
