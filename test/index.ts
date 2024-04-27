import { DrawingContext, Dugtrio } from "..";

Dugtrio.init();

setInterval(() => {
  DrawingContext.circle({
    position: { x: 100, y: 150 },
    fill: true,
    radius: 45,
  });
  DrawingContext.color({
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
  });
  DrawingContext.circle({
    position: { x: 130, y: 150 },
    fill: true,
    radius: 45,
  });
  Dugtrio.draw();
}, 4);
