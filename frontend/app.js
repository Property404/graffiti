"use strict";
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d")
const size = 1024;
const sse = new EventSource("/api/feed");

sse.addEventListener("message", (e) => {
    console.log("Ooh a message!")
    console.log(e.data);
});

let color = {red: 255, green:0, blue:0};
let radius = 50;

function form_update_from_rect(x, y, r) {
    return {
        start: {
            x: x - r,
            y: y - r,
        },
        end: {
            x: x - r,
            y: y - r,
        },
        color: color
    }
}

function send_update(update) {
    fetch('/api/update', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(update)
    })
}

function apply_update() {
}

canvas.addEventListener('click', function(e) {
    const mousex = size * (e.offsetX / canvas.clientWidth)
    const mousey = size * (e.offsetY / canvas.clientHeight)
    const update = form_update_from_rect(mousex, mousey, radius);
    console.log(update);
    send_update(update);
    //ctx.fillRect(mousex-radius, mousey-radius, 2*radius, 2*radius);
});
