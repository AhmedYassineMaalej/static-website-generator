const socket = new WebSocket("ws://127.0.0.1:9001");
const content = document.querySelector(".content");
const style = document.querySelector("style");


socket.addEventListener("open", e => console.log("connected to server: ", e));

socket.addEventListener("message", m => {
    console.log("received data");
    let msg = JSON.parse(m.data);
    console.log(msg);
    if (msg.type == "css") {
        style.innerHTML = msg.payload;
    }
    if (msg.type == "html") {
        content.innerHTML = msg.payload;
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
