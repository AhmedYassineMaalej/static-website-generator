const socket = new WebSocket("ws://127.0.0.1:9001");
const contentContainer = document.querySelector(".content-container");
const indexContainer = document.querySelector(".index-container");
const titleContainer = document.querySelector(".title-container");
const tagsContainer = document.querySelector(".tags-container");
const style = document.querySelector("#hot-css");

socket.addEventListener("open", e => console.log("connected to server: ", e));

socket.addEventListener("message", m => {
    console.log("received data");
    let msg = JSON.parse(m.data);

    if (msg.type == "css") {
        style.innerHTML = msg.css;
    }

    if (msg.type == "markdown") {
        contentContainer.innerHTML = msg.content;
        indexContainer.innerHTML = msg.index;
        titleContainer.innerHTML = msg.title;
        tagsContainer.innerHTML = msg.tags;
        MathJax.typeset();
    }

    if (msg.type == "html") {
        window.location.reload(true);
    }
});

socket.addEventListener("close", _ => console.log("disconnected from server"));

window.addEventListener('beforeunload', (_event) => {
    socket.close();
});

contentContainer.addEventListener("click", e => {
    const position = e.target.dataset.position;
    socket.send(position);
});
