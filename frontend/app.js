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
        let old_x = null;
        let old_y = null;
        const draw = function(e) {
            const mouse_x = canvas_width * (e.offsetX / canvas.clientWidth)
            const mouse_y = canvas_height * (e.offsetY / canvas.clientHeight)
            const updates = [
                form_update_from_rect(mouse_x, mouse_y, radius),
            ]

            const num_steps = 4;
            const delta_x = (mouse_x - old_x) / num_steps;
            const delta_y = (mouse_y - old_y) / num_steps;

            if (old_x !== null) {
                for (let step = 1; step < num_steps; step++) {
                    updates.push(form_update_from_rect(old_x + delta_x * step, old_y + delta_y * step, radius))
                }
            }

            for (const update of updates) {
                apply_update(update);
                send_update(update);
            }

            old_x = mouse_x;
            old_y = mouse_y;
        };

        draw(e);
        canvas.onmousemove = draw;

        canvas.onmouseup = () => {
            canvas.onmousemove = null;
            canvas.onmouseup = null;
        }
        canvas.onmouseleave = () => {
            old_x = null;
            old_y = null;
        }
    });

    console.log("Setting up SSE");

    const sse = new EventSource("/api/feed");
    sse.addEventListener("message", (e) => {
        apply_update(JSON.parse(e.data));
    });

    console.log("Ready!");
}

main()
