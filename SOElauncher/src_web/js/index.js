var responseContainer = document.getElementById("response");

const invoke = window.__TAURI__.invoke;
var windowId = Math.random().toString().replace(".", "");
var windowNumber = 1;

invoke("correct_pos").then((cor) =>{
    if (cor){
        setInterval(load_mods(), 5000)
    }
    else{
        popup("comp/fetcherr")
    }
})

function load_mods() {
    invoke("fetch_modlist").then((modlist) => {
        let htmllist = document.getElementById("modlist");
        htmllist.innerHTML = " ";
        console.log(modlist);
        console.log(modlist.length);

        for (let i = 0; i <= modlist.length - 1; i++) {
            console.log("adding list");
            let li = document.createElement("li");
            let id = "buttonmod" + i;
            //<hr class="solid">
            let code = 
            `<div class="modrow">
                <a id="`+id+"sub"+`">`+modlist[i]+` </a>
                
                <button class="mod_button red" id="`+id+`" onclick='mod("`+id+`")'> Not on</button>
                
            </div>
            
            
            `

            li.innerHTML = code;
            htmllist.appendChild(li);

        }
        console.log("done");
    });
}
var actmods = [];
function mod(btn) {
    let btnid = document.getElementById(btn);

    let pth = document.getElementById(btn + "sub").innerText;

    for (let i = 0; i <= actmods.length - 1; i++) {
        if (pth == actmods[i]) {
            btnid.innerText = "Not on";
            btnid.className= "mod_button red";
            actmods.splice(i, 1);

            return;
        }
    }

    actmods.push(pth);
    console.log("pt" + pth);
    console.log(actmods);
    btnid.className= "mod_button green";
    btnid.innerText = "On";
}

function test() {
    invoke("test");
}
function idSetInner(id, text) {
    document.getElementById(id).innerHTML = text;
};


function close_popup() {
    document.getElementById("portal").innerHTML = "";
}
function load(component){
    return fetch(component + ".html").then((response) => response.text());
}
function popup(strs) {
    
    invoke("report_backend", { data: strs })
        .then((response) => {
            str = response;
            console.log("ok" + response);
            load(str).then((text) => idSetInner("portal", text));
        })
        .catch((error) => {
            console.log("err" + error);
        });

    
}

function open_web() {
    invoke("open_web");
}
function open_git() {
    invoke("open_web_git");
}

function start_game() {
    invoke("start", { data: actmods });
}

window.__TAURI__.event.listen("tauri://window-created", function (event) {
    responseContainer.innerText += "Got window-created event\n\n";
});
