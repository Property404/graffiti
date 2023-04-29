"use strict";
const canvas = document.querySelector("canvas");
const loading_text = document.getElementById("loading-text");
const main_container = document.getElementById("main");
const ctx = canvas.getContext("2d")
const canvas_width = 1024;
const canvas_height = 1024;
canvas.width = canvas_width
canvas.height = canvas_height

let color = {
    red: 0,
    green: 0,
    blue: 0
};
let radius = 10;
let shape = "square";

function rgbToHex(r, g, b) {
    function componentToHex(c) {
        const hex = c.toString(16);
        return hex.length == 1 ? "0" + hex : hex;
    }
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
}

function hexToRgb(hex) {
    const val = parseInt(hex.substr(1), 16);
    return {
        red: (val >> 16) & 255,
        green: (val >> 8) & 255,
        blue: val & 255,
    }
}

function form_update(x, y, radius, shape) {
    x = Math.floor(x)
    y = Math.floor(y)
    radius = Math.floor(radius)
    return {
        x,
        y,
        radius,
        color,
        shape
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

function update_brush() {
    const color_value = document.getElementById("color-button").value;
    radius = document.getElementById("radius-slider").value;
    color = hexToRgb(color_value);
}

function apply_update(update) {
    const x = update?.x;
    const y = update?.y;
    const radius = update?.radius;
    if (x == null || y == null || radius == null || radius < 0 || radius >= canvas_width) {
        console.log("Bad update: ", update);
        return;
    }

    if (update?.color !== null) {
        const hex = rgbToHex(color.red, color.green, color.blue);
        ctx.fillStyle = hex;
    }
    if (update.shape === "square") {
        ctx.fillRect(x - radius, y - radius, radius * 2, radius * 2)
    } else if (update.shape === "circle") {
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.fill();
    }
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

function draw_shape(old_x, old_y, mouse_x, mouse_y) {
    const updates = [
        form_update(mouse_x, mouse_y, radius, shape),
    ]

    const num_steps = 4;
    const delta_x = (mouse_x - old_x) / num_steps;
    const delta_y = (mouse_y - old_y) / num_steps;

    if (old_x !== null) {
        for (let step = 1; step < num_steps; step++) {
            updates.push(form_update(old_x + delta_x * step, old_y + delta_y * step, radius, shape))
        }
    }

    for (const update of updates) {
        apply_update(update);
        send_update(update);
    }

}

function calc_mouse_positions(e) {
    const mouse_x = canvas_width * (e.offsetX / canvas.clientWidth);
    const mouse_y = canvas_height * (e.offsetY / canvas.clientHeight);
    return [mouse_x, mouse_y];
}

async function main() {
    restoreCanvas().await;

    update_brush();

    canvas.addEventListener("mousedown", function(e) {
        let old_x = null;
        let old_y = null;

        const draw = function(e) {
            const [mouse_x, mouse_y] = calc_mouse_positions(e)
            draw_shape(old_x, old_y, mouse_x, mouse_y);
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

    document.getElementById("radius-slider").onchange = update_brush;
    document.getElementById("color-button").onchange = update_brush;

    const sse = new EventSource("/api/feed");
    sse.addEventListener("message", (e) => {
        apply_update(JSON.parse(e.data));
    });

    // Show canvas/toolbar and hide loading text
    loading_text.setAttribute("hidden", true);
    main_container.style = "";
}

main()
