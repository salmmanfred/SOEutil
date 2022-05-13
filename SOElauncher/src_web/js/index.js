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

    
}
function popup(str) {
    console.log("startpop");
    load(str).then(text => idSetInner("portal", text));
    console.log("end pop");
    
}

function open_web(){
    invoke("open_web_git");
}

function start_web(){
    invoke("start");

}


window.__TAURI__.event.listen('tauri://window-created', function (event) {
    responseContainer.innerText += 'Got window-created event\n\n'
})