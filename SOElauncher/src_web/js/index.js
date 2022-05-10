var responseContainer = document.getElementById('response')

const invoke = window.__TAURI__.invoke
var windowId = Math.random().toString().replace('.', '')
var windowNumber = 1
function test() {
    invoke("test")
}
const idSetInner = (id, text) => {
    document.getElementById(id).innerHTML = text;
}
const load = (component) => {
    return fetch(component + '.html')
        .then(response => response.text());

    // ! If we ever want to add lazy loading for css & js
    // const head = document.getElementsByTagName("head")[0];
    // if (!document.getElementById(component + "-css")) {
    //     const cssLink = document.createElement("link");

    //     cssLink.href = "css/" + component + ".css";
    //     cssLink.id = component + "-css";
    //     cssLink.type = "text/css";
    //     cssLink.rel = "stylesheet";

    //     head.appendChild(cssLink);
    // }
    // if (!document.getElementById(component + "-js")) {
    //     const jsLink = document.createElement("script");

    //     jsLink.src = "js/" + component + ".js";
    //     jsLink.id = component + "-js";
    //     jsLink.defer = true;

    //     head.appendChild(cssLink);
    // }
}
function popup() {
    load("popup").then(text => idSetInner("portal", text));
    // invoke("popup", { id: `child-${windowId}-${windowNumber}`,typ:1 }).then((response) => {
    //     responseContainer.innerText += `Ok(${response})\n\n`
    // })
    // .catch((error) => {
    //     responseContainer.innerText += `Err(${error})\n\n`
    // });
    // windowNumber += 1;
}
window.__TAURI__.event.listen('tauri://window-created', function (event) {
    responseContainer.innerText += 'Got window-created event\n\n'
})