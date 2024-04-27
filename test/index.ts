import { DrawingContext, Dugtrio } from "..";
import { Interactable } from "../src/Interactable";
Dugtrio.init();

const window = new Interactable();
window.draw = (self) => {};

const box = new Interactable();
box.size = { x: 100, y: 100 };
box.position = { x: 100, y: 100 };
box.draw = (self) => {
  const pos = Dugtrio.getCursorPosition();
  self.position = pos;
  DrawingContext.rect({
    position: self.position,
    fill: false,
    size: self.size,
  });
};

window.child(box);

setInterval(() => {
  window.render();
  Dugtrio.draw();
}, 4);
