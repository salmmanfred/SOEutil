var responseContainer = document.getElementById('response')

const invoke = window.__TAURI__.invoke
var windowId = Math.random().toString().replace('.', '')
var windowNumber = 1

load_mods()
function load_mods(){
    invoke("fetch_modlist").then((modlist) =>{
        let htmllist = document.getElementById("modlist")
        htmllist.innerHTML = " ";
        console.log(modlist)
        console.log(modlist.length)

        for (let i = 0; i <= modlist.length-1; i++){
            console.log("adding list");
            let li = document.createElement("li");
            let intdiv = document.createElement("div");
            let intdiv2 = document.createElement("div");
            let id = "buttonmod"+i;

            intdiv2.appendChild(document.createTextNode(modlist[i]))
            intdiv2.id = id+"sub"
            intdiv.appendChild(intdiv2)
            let intbutton = document.createElement("button")
            intbutton.innerText = "Not on";
            intbutton.setAttribute("onclick", "mod(\""+id+"\")");
            intbutton.id = id
            
            intdiv.appendChild(intbutton);

            li.appendChild(intdiv);
            htmllist.appendChild(li);
        }
        console.log("done");


    });
    
}
var actmods = [];
function mod(btn){
    let btnid = document.getElementById(btn);
    
    let pth = document.getElementById(btn+"sub").innerText;
    

    for (let i = 0; i <= actmods.length-1; i++){
        if (pth == actmods[i]){
            btnid.innerText = "Not on"
            actmods.splice(i,1);
            return;
        }
    }


    actmods.push(pth);
    console.log("pt" + pth);
    console.log(actmods)
    btnid.innerText = "On"


}

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
function call_back(strs){
    
   
    
}

function popup(strs) {
    console.log("startpop");
    invoke("report_backend", { data: strs}).then((response) => {
          str = response
          console.log("ok" + response)
          load(str).then(text => idSetInner("portal", text));
        
    }).catch((error) => {
          console.log("err" + error);
          
    });
    
    
    console.log("end pop");
    
}

function open_web(){
    invoke("open_web_git");
}

function start_game(){
    invoke("start", {data: actmods});

}


window.__TAURI__.event.listen('tauri://window-created', function (event) {
    responseContainer.innerText += 'Got window-created event\n\n'
})