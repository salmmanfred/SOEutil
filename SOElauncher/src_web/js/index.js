const invoke = window.__TAURI__.invoke;

var responseContainer = document.getElementById("response");
var settings = {
    active_mods: []
}

invoke("get_launcher_settings").then(data => settings = data)

invoke("correct_pos").then((cor) =>{
    if (cor){
        setInterval(load_mods, 5000)
        load_mods()
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

        for (let i = 0; i < modlist.length; i++) {
            console.log("adding list");
            const isModLoaded = settings.active_mods.includes(modlist[i])
            const li = document.createElement("li");
            li.innerHTML =
                    `
                    <li>
                        <div class="modrow">
                            <p id="buttonmod${i}sub" class="modname"> ${modlist[i]} </p>
                            <button 
                                id="buttonmod${i}" 
                                class="mod_button ${isModLoaded?"green":"red"}" 
                                onclick="mod('buttonmod${i}')"
                            >
                                ${isModLoaded?"On":"Off"}
                            </button>
                        </div>
                    </li>

                    `;

            htmllist.appendChild(li);
        }
        console.log("done");
    });
}

function mod(btn) {
    let btnid = document.getElementById(btn);

    let pth = document.getElementById(btn + "sub").innerText;

    for (let i = 0; i < settings.active_mods.length; i++) {
        if (pth == settings.active_mods[i]) {
            btnid.innerText = "Not on";
            btnid.classList.replace("green", "red");
            settings.active_mods.splice(i, 1);
    invoke("save_launcher_settings", {settings: settings});
            

            return;
        }
    }

    settings.active_mods.push(pth);
    console.log("pt" + pth);
    console.log(settings.active_mods);
    btnid.classList.replace("red", "green");
    btnid.innerText = "On";
    invoke("save_launcher_settings", {settings: settings});

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

    invoke("start", { data: settings.active_mods });
}

window.__TAURI__.event.listen("tauri://window-created", function (event) {
    responseContainer.innerText += "Got window-created event\n\n";
});
