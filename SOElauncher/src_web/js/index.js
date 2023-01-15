const invoke = window.__TAURI__.invoke

var response_container = document.getElementById("response")
var settings = {
    active_mods: [],
}

invoke("get_launcher_settings").then((data) => (settings = data))

invoke("correct_pos").then((cor) => {
    if (cor) {
        setInterval(load_mods, 5000)
        load_mods()
    } else {
        popup("comp/fetcherr")
    }
})

function load_mods() {
    invoke("fetch_modlist").then((modlist) => {
        let htmllist = document.getElementById("modlist")
        htmllist.innerHTML = " "
        console.log(modlist)
        console.log(modlist.length)

        for (let i = 0; i < modlist.length; i++) {
            console.log("adding list")
            const is_mod_on = settings.active_mods.includes(modlist[i])
            const li = document.createElement("li")
            li.innerHTML = `
                    <li>
                        <div class="modrow">
                            <p id="buttonmod${i}sub" class="modname"> ${modlist[i]} </p>
                            <button 
                                id="buttonmod${i}" 
                                class="mod_button ${is_mod_on ? "green" : "red"}" 
                                onclick="toggle_mod('buttonmod${i}')"
                            >
                                ${is_mod_on ? "On" : "Off"}
                            </button>
                        </div>
                    </li>
                    `

            htmllist.appendChild(li)
        }
        console.log("done")
    })
}

function toggle_mod(buttonId) {
    const button = document.getElementById(buttonId)
    const mod_name = document.getElementById(buttonId + "sub").innerText

    const index = settings.active_mods.indexOf(mod_name)

    if (index == -1) {
        button.classList.replace("red", "green")
        button.innerText = "On"
        settings.active_mods.push(mod_name)
    } else {
        button.innerText = "Not on"
        button.classList.replace("green", "red")
        settings.active_mods.splice(index, 1)
    }

    console.log("pt" + path)
    console.log(settings.active_mods)
}

function test() {
    invoke("test")
}

function close_popup() {
    document.getElementById("portal").innerHTML = ""
}
function load(component) {
    return fetch(component + ".html").then((response) => response.text())
}
function popup(strs) {
    invoke("report_backend", { data: strs })
        .then((response) => {
            str = response
            console.log("ok" + response)
            load(str).then((text) => (document.getElementById("portal").innerText = text))
        })
        .catch((error) => {
            console.log("err" + error)
        })
}
;+function open_web() {
    invoke("open_web")
}
function open_git() {
    invoke("open_web_git")
}

function start_game() {
    invoke("start", { data: settings.active_mods })
}

window.__TAURI__.event.listen("tauri://window-created", function (event) {
    response_container.innerText += "Got window-created event\n\n"
})
