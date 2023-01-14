const invoke = window.__TAURI__.invoke;

var responseContainer = document.getElementById("response");
var settings = {
    previous_mods: []
}

invoke("get_launcher_settings").then(data => settings = data)

invoke("correct_pos").then((cor) =>{
    if (cor){
        setInterval(load_mods, 5000)
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
        console.log(settings)

        for (let i = 0; i <= modlist.length - 1; i++) {
            console.log("adding list");
            const li = document.createElement("li");

            const modRow = document.createElement("div")
            modRow.classList.add("modrow")

            const p = document.createElement("p")
            p.setAttribute("id", `buttonmod${i}sub`)
            p.innerText = modlist[i]
            p.classList.add("modname")

            const button = document.createElement("button")
            button.classList.add("mod_button")
            button.setAttribute("id", `buttonmod${i}`)
            button.onclick = ()=> {
                mod(button.getAttribute("id"))
            }

            if(settings.previous_mods.includes(modlist[i])) {
                button.innerText = "On"
                button.classList.add("green")
            } else {
                button.innerText = "Not on"
                button.classList.add("red")
            }

            modRow.appendChild(p)
            modRow.appendChild(button)

            li.appendChild(modRow)
            htmllist.appendChild(li);

        }
        console.log("done");
    });
}

function mod(btn) {
    let btnid = document.getElementById(btn);

    let pth = document.getElementById(btn + "sub").innerText;

    for (let i = 0; i <= settings.previous_mods.length - 1; i++) {
        if (pth == settings.previous_mods[i]) {
            btnid.innerText = "Not on";
            btnid.classList.replace("green", "red");
            settings.previous_mods.splice(i, 1);

            return;
        }
    }

    settings.previous_mods.push(pth);
    console.log("pt" + pth);
    console.log(settings.previous_mods);
    btnid.classList.replace("red", "green");
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
    invoke("start", { data: settings.previous_mods });
}

window.__TAURI__.event.listen("tauri://window-created", function (event) {
    responseContainer.innerText += "Got window-created event\n\n";
});
