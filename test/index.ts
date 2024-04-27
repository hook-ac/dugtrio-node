import { DrawingContext, Dugtrio } from "..";
import { draggable } from "../plugins/draggable";
import { mouseOver } from "../plugins/mouseOver";
import { Interactable } from "../src/Interactable";
Dugtrio.init();

const window = new Interactable();
window.draw = (self) => {};

const box = new Interactable();
box.size = { x: 100, y: 100 };
box.position = { x: 100, y: 100 };
box.draw = (self) => {
  if (self.properties.mouseOver) {
    DrawingContext.color({ red: 255, blue: 0, alpha: 255, green: 255 });
  } else {
    DrawingContext.color({ red: 255, blue: 0, alpha: 255, green: 0 });
  }
  DrawingContext.text({
    position: self.position,
    text: "hello",
  });

  DrawingContext.rect({
    position: self.position,
    fill: false,
    size: self.size,
  });
};
box.addPlugin(mouseOver());
box.addPlugin(draggable());

window.child(box);

setInterval(() => {
  window.render();
  Dugtrio.draw();
}, 4);
