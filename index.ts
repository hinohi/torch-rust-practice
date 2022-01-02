import {App, MouseEventType} from "torch-onnx-web-rs";
const canvas = document.getElementById('canvas') as HTMLCanvasElement | null;
if (!canvas) {
    throw new Error('No "canvas" element');
}
canvas.width = 600;
canvas.height = 600;

const context = canvas.getContext('2d');
if (!context) {
    throw new Error('Unsupported 2d');
}

const app = new App(context);

canvas.addEventListener('mousedown', (event) => {
    app.mouse_event(MouseEventType.Down, event.offsetX, event.offsetY);
});
canvas.addEventListener('mouseup', (event) => {
    app.mouse_event(MouseEventType.Up, event.offsetX, event.offsetY);
});
canvas.addEventListener('mousemove', (event) => {
    app.mouse_event(MouseEventType.Move, event.offsetX, event.offsetY);
});
canvas.addEventListener('mouseenter', (event) => {
    app.mouse_event(MouseEventType.Enter, event.offsetX, event.offsetY);
});
canvas.addEventListener('mouseleave', (event) => {
    app.mouse_event(MouseEventType.Leave, event.offsetX, event.offsetY);
});
canvas.addEventListener('mouseover', (event) => {
    app.mouse_event(MouseEventType.Over, event.offsetX, event.offsetY);
});
canvas.addEventListener('mouseout', (event) => {
    app.mouse_event(MouseEventType.Out, event.offsetX, event.offsetY);
});
