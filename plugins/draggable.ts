import { Dugtrio } from "..";
import { Interactable, PluginResponse } from "../src/Interactable";

export function draggable(): PluginResponse {
  return {
    setup: (self: Interactable) => {
      if (!self.properties.pluginMouseOver) {
        throw "Draggable requires mouseOver.";
      }
      self.properties.lastPressing = false;
      self.properties.delta = { x: 0, y: 0 };
      self.properties.isDragging = false;
    },
    draw: (self: Interactable) => {
      const isOver = self.properties.mouseOver;
      const lastPressing = self.properties.lastPressing;
      const isPressing = Dugtrio.isMouseDown(0);

      if (isOver && lastPressing !== isPressing) {
        self.properties.lastPressing = isPressing;

        if (isPressing) {
          const cursorPosition = Dugtrio.getCursorPosition();

          self.properties.delta = {
            x: self.position.x - cursorPosition.x,
            y: self.position.y - cursorPosition.y,
          };
          self.properties.isDragging = true;
        } else {
          self.properties.isDragging = false;
        }
      }

      if (self.properties.isDragging) {
        const cursorPosition = Dugtrio.getCursorPosition();
        self.position.x = cursorPosition.x + self.properties.delta.x;
        self.position.y = cursorPosition.y + self.properties.delta.y;
      }
    },
  };
}
