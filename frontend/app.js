"use strict";
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d")
const canvas_width = canvas.width;
const canvas_height = canvas.height;

let color = {
    red: 255,
    green: 0,
    blue: 0
};
let radius = 10;

function rgbToHex(r, g, b) {
    function componentToHex(c) {
        const hex = c.toString(16);
        return hex.length == 1 ? "0" + hex : hex;
    }
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
}

function form_update_from_rect(x, y, r) {
    x = Math.floor(x)
    y = Math.floor(y)
    r = Math.floor(r)
    return {
        start: {
            x: Math.max(x - r, 0),
            y: Math.max(y - r, 0),
        },
        end: {
            x: Math.min(x + r, canvas_width),
            y: Math.min(y + r, canvas_height),
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
    if (start_x == null || start_y == null || start_x >= canvas_width || start_x > end_x ||
        start_y >= canvas_height || start_y > end_y) {
        console.log("Bad update");
        return;
    }

    if (color !== null) {
        const hex = rgbToHex(color.red, color.green, color.blue);
        ctx.fillStyle = hex;
    }
    ctx.fillRect(start_x, start_y, end_x - start_x, end_y - start_y);
}

async function restoreCanvas() {
    const state = new Uint32Array(await (await fetch("/api/state")).arrayBuffer());
    const canvas_data = ctx.getImageData(0, 0, canvas.width, canvas.height);
    let red, green, blue = 0;
    for (let code of state) {
        if (code & 0x80000000) {
            code &= ~0x80000000;
            const x = code >> 16;
            const y = code & 0xFFFF;
            const index = (x + y * canvas.width) * 4;
            canvas_data.data[index + 0] = red;
            canvas_data.data[index + 1] = green;
            canvas_data.data[index + 2] = blue;
            canvas_data.data[index + 3] = 255;
        } else {
            red = (code >> 16) & 0xFF;
            green = (code >> 8) & 0xFF;
            blue = code & 0xFF;
        }
    }
    ctx.putImageData(canvas_data, 0, 0);
}

async function main() {
    console.log("Loading previous state...");

    restoreCanvas().await;

    console.log("Setting up event listeners...");


    canvas.addEventListener("mousedown", function(e) {
        const draw = function(e) {
            const mousex = canvas_width * (e.offsetX / canvas.clientWidth)
            const mousey = canvas_height * (e.offsetY / canvas.clientHeight)
            const update = form_update_from_rect(mousex, mousey, radius);
            apply_update(update);
            send_update(update);
        };

        draw(e);
        canvas.onmousemove = draw;

        canvas.onmouseup = function() {
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
