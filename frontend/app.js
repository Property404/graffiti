"use strict";
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d")
const size = 1024;
const sse = new EventSource("/api/feed");

sse.addEventListener("message", (e) => {
    apply_update(JSON.parse(e.data));
});

let color = {red: 255, green:0, blue:0};
let radius = 10;
let listener = null;

function form_update_from_rect(x, y, r) {
    return {
        start: {
            x: Math.max(x - r, 0),
            y: Math.max(y - r, 0),
        },
        end: {
            x: Math.min(x + r, size),
            y: Math.min(y + r, size),
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
    const start_x = update?.start?.x;
    const end_x = update?.end?.x;
    const start_y = update?.start?.y;
    const end_y = update?.end?.y;
    if (start_x == null || start_y == null || start_x >= size || start_x > end_x
        || start_y >= size || start_y > end_y ) {
        console.log("Bad update");
        return;
    }

    ctx.fillRect(start_x, start_y, end_x - start_x, end_y - start_y);
}

canvas.addEventListener("mousedown", function() {
    if (listener === null) {
        canvas.onmousemove = function(e) {
            const mousex = size * (e.offsetX / canvas.clientWidth)
            const mousey = size * (e.offsetY / canvas.clientHeight)
            const update = form_update_from_rect(mousex, mousey, radius);
            ctx.fillStyle = "cyan";
            apply_update(update);
            ctx.fillStyle = "black";
            send_update(update);
        };
        canvas.onmouseup = function() {
            console.log("bye");
            canvas.onmousemove = null;
            canvas.onmouseup = null;
        };
    }
});