"use strict";
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d")
const size = 1024;

let radius = 50;

canvas.addEventListener('click', function(e) {
    const mousex = size * (e.offsetX / canvas.clientWidth)
    const mousey = size * (e.offsetY / canvas.clientHeight)
    console.log(mousex);
    ctx.fillRect(mousex-radius, mousey-radius, 2*radius, 2*radius);
});
