var bod = document.getElementById('bod')

invoke("get_cont").then((response) => {
    bod.innerText += `Ok(${response})\n\n`
})
.catch((error) => {
    bod.innerText += `Err(${error})\n\n`
});