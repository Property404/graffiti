"use strict";
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d")
const size = 1024;

let color = {red: 255, green:0, blue:0};
let radius = 10;

function rgbToHex(r, g, b) {
    function componentToHex(c) {
        const hex = c.toString(16);
        return hex.length == 1 ? "0" + hex : hex;
    }
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
}

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

    if (color !== null) {
        const hex = rgbToHex(color.red, color.green, color.blue);
        ctx.fillStyle = hex;
    }
    ctx.fillRect(start_x, start_y, end_x - start_x, end_y - start_y);
}

async function main() {
    console.log("Loading previous state...");
    const state = await (await fetch("/api/state")).json();

    for (const pair of state) {
        ctx.fillStyle = rgbToHex(pair[1].red,pair[1].green,pair[1].blue);
        ctx.fillRect(pair[0].x, pair[0].y, 1, 1);
    }


    console.log("Setting up event listeners...");


    canvas.addEventListener("mousedown", function(e) {
        const draw = function(e) {
            const mousex = size * (e.offsetX / canvas.clientWidth)
            const mousey = size * (e.offsetY / canvas.clientHeight)
            const update = form_update_from_rect(mousex, mousey, radius);
            apply_update(update);
            send_update(update);
        };

        draw(e);
        canvas.onmousemove = draw;

        canvas.onmouseup = function() {
            console.log("bye");
            canvas.onmousemove = null;
            canvas.onmouseup = null;
        };
    });

    console.log("Setting up SSE");

    const sse = new EventSource("/api/feed");
    sse.addEventListener("message", (e) => {
        apply_update(JSON.parse(e.data));
    });

    console.log("Ready!");
}

main()
