import { Dugtrio } from "..";
import { Interactable, PluginResponse } from "../src/Interactable";

export function mouseOver(): PluginResponse {
  return {
    setup: (self: Interactable) => {
      self.properties.pluginMouseOver = true;
      self.properties.mouseOver = false;
    },
    draw: (self: Interactable) => {
      const mousePosition = Dugtrio.getCursorPosition();
      if (
        mousePosition.x > self.position.x &&
        mousePosition.y > self.position.y &&
        mousePosition.x < self.position.x + self.size.x &&
        mousePosition.y < self.position.y + self.size.y
      ) {
        self.properties.mouseOver = true;
      } else {
        self.properties.mouseOver = false;
      }
    },
  };
}
