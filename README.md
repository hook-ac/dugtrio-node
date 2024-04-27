# Dugtrio

A small drawing framework for osu!lazer with Node.JS backend.

## How to use

1. Open osu!lazer
2. Run `native/dugtrio.exe`
3. Run a drawing nodejs code

Example:

```typescript
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
```

4. Press HOME in game to see that drawing context

![](/assets/screenshot.png)

## Notice

- dugtrio.exe is a simple injector, if you want to embed, please use your own injector or manual map and load the dll.
