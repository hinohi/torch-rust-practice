import {App, MouseEventType} from "torch-onnx-web-rs";
const canvas = document.getElementById('canvas') as HTMLCanvasElement | null;
if (!canvas) {
    throw new Error('No "canvas" element');
}

const mnistSize = 28;
const canvasSize = Math.floor(400 / mnistSize) * mnistSize;
canvas.width = canvasSize;
canvas.height = canvasSize;

const context = canvas.getContext('2d');
if (!context) {
    throw new Error('Unsupported 2d');
}

const app = new App(canvasSize, context);

canvas.addEventListener('mousedown', (event) => {
    app.mouse_event(MouseEventType.Down, event.offsetX, event.offsetY, event.buttons);
});
canvas.addEventListener('mouseup', (event) => {
    app.mouse_event(MouseEventType.Up, event.offsetX, event.offsetY, event.buttons);
});
canvas.addEventListener('mousemove', (event) => {
    app.mouse_event(MouseEventType.Move, event.offsetX, event.offsetY, event.buttons);
});
canvas.addEventListener('mouseenter', (event) => {
    app.mouse_event(MouseEventType.Enter, event.offsetX, event.offsetY, event.buttons);
});
canvas.addEventListener('mouseleave', (event) => {
    app.mouse_event(MouseEventType.Leave, event.offsetX, event.offsetY, event.buttons);
});
