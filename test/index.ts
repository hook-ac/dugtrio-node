import { DrawingContext, Dugtrio } from "..";
import { draggable } from "../plugins/draggable";
import { mouseOver } from "../plugins/mouseOver";
import { pin } from "../plugins/pin";
import { Interactable } from "../src/Interactable";
Dugtrio.init("opengl", "x32");

const window = new Interactable();
window.draw = (self) => {};

const boxHead = new Interactable();
boxHead.size = { x: 100, y: 25 };
boxHead.position = { x: 100, y: 100 };

boxHead.draw = (self) => {
  if (self.properties.mouseOver) {
    DrawingContext.color({ red: 255, blue: 0, alpha: 255, green: 255 });
  } else {
    DrawingContext.color({ red: 255, blue: 0, alpha: 255, green: 0 });
  }
  DrawingContext.fontSize({ value: 99 });
  DrawingContext.fontAlign({ value: 1 });
  DrawingContext.text({
    position: self.position,
    text: "hellos",
  });

  DrawingContext.rect({
    position: self.position,
    fill: false,
    size: self.size,
  });
};

boxHead.addPlugin(mouseOver());
boxHead.addPlugin(draggable());

const box = new Interactable();
box.size = { x: 100, y: 100 };
box.position = { x: 100, y: 100 };

box.draw = (self) => {
  DrawingContext.color({ red: 255, blue: 0, alpha: 255, green: 0 });

  DrawingContext.rect({
    position: self.position,
    fill: false,
    size: self.size,
  });
};
boxHead.child(box);
box.addPlugin(pin());

window.child(boxHead);

setInterval(() => {
  // DrawingContext.block({ value: true });
  window.render();
  Dugtrio.draw();
}, 4);
