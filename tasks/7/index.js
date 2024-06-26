import { World, Direction } from "rust-snake";

const canvas = document.getElementById("myCanvas");
const ctx = canvas.getContext("2d");
const cell_size = 15;

function draw_canvas() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

function draw_snake(body) {
    for (const cell of body) {
        ctx.fillRect(cell.x * cell_size, cell.y * cell_size, cell_size, cell_size);
    }
}

function gameloop(world) {
    world.update();
    const body = world.get_body();

    draw_canvas();
    draw_snake(body);
    setTimeout(() => gameloop(world), 100);
}

const world = World.new();
canvas.height = world.get_height() * cell_size;
canvas.width = world.get_width() * cell_size;

document.addEventListener("keydown", function(event) {
    switch(event.key) {
        case "ArrowLeft":
            world.set_direction(Direction.Left);
            break;
        case "ArrowRight":
            world.set_direction(Direction.Right);
            break;
        case "ArrowUp":
            world.set_direction(Direction.Up);
            break;
        case "ArrowDown":
            world.set_direction(Direction.Down);
            break;
    }
});

gameloop(world);