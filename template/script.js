const socket = new WebSocket("ws://127.0.0.1:9001");
const content = document.querySelector(".content");
const index = document.querySelector(".index");
const style = document.querySelector("style");


socket.addEventListener("open", e => console.log("connected to server: ", e));

socket.addEventListener("message", m => {
    console.log("received data");
    let msg = JSON.parse(m.data);

    if (msg.type == "css") {
        style.innerHTML = msg.css;
    }

    if (msg.type == "markdown") {
        content.innerHTML = msg.content;
        index.innerHTML = msg.index;
    }

    if (msg.type == "html") {
        window.location.reload(true);
    }
});

socket.addEventListener("close", _ => console.log("disconnected from server"));

window.addEventListener('beforeunload', (_event) => {
    socket.close();
});

content.addEventListener("click", e => {
    const position = e.target.dataset.position;
    socket.send(position);
});
