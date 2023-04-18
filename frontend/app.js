"use strict";
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d")
const size = 1024;
const sse = new EventSource("/api/feed");

sse.addEventListener("message", (e) => {
    console.log("Ooh a message!")
    console.log(e.data);
    apply_update(JSON.parse(e.data));
});

let color = {red: 255, green:0, blue:0};
let radius = 10;

function form_update_from_rect(x, y, r) {
    return {
        start: {
            x: x - r,
            y: y - r,
        },
        end: {
            x: x + r,
            y: y + r,
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

function apply_update(update) {
    console.log(update);
    const start_x = update?.start?.x;
    const end_x = update?.end?.x;
    const start_y = update?.start?.y;
    const end_y = update?.end?.y;
    console.log(start_x, start_y, end_x - start_x, end_y - start_y);
    if (start_x == null || start_y == null || start_x < 0 || start_x >= size || start_x > end_x
        || start_y < 0 || start_y >= size || start_y > end_y ) {
        console.log("Bad update");
        return;
    }

    console.log("Applying")
    ctx.fillRect(start_x, start_y, end_x - start_x, end_y - start_y);
}

canvas.addEventListener('mousedown', function(e) {
    const mousex = size * (e.offsetX / canvas.clientWidth)
    const mousey = size * (e.offsetY / canvas.clientHeight)
    const update = form_update_from_rect(mousex, mousey, radius);
    ctx.fillStyle = "pink";
    console.log(update);
    send_update(update);
    ctx.fillStyle = "blue";
    ctx.fillRect(mousex-radius, mousey-radius, 2*radius, 2*radius);
});
