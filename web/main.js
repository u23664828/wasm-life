import init, { Game } from "./pkg/wasm_life.js";
await init();

const boardEl = document.getElementById("board");
const statusEl = document.getElementById("status");
const resetBtn = document.getElementById("reset");

const game = new Game();

// build 9 clickable cells
const cells = [];
for (let i = 0; i < 9; i++) {
  const div = document.createElement("div");
  div.className = "cell";
  div.dataset.idx = i;
  div.onclick = () => {
    if (game.play(i)) render();
  };
  boardEl.appendChild(div);
  cells.push(div);
}

resetBtn.onclick = () => { game.reset(); render(); };

function render() {
  const b = game.get_board(); // Uint8Array
  for (let i = 0; i < 9; i++) {
    cells[i].textContent = b[i] === 1 ? "X" : b[i] === 2 ? "O" : "";
  }
  const w = game.winner();
  if (w === 1) statusEl.textContent = "X wins!";
  else if (w === 2) statusEl.textContent = "O wins!";
  else if (w === 3) statusEl.textContent = "Draw!";
  else statusEl.textContent = `Turn: ${game.current_player() === 1 ? "X" : "O"}`;
}

render();
