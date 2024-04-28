import { Dugtrio } from "..";
import { Interactable, PluginResponse } from "../src/Interactable";

export function pin(): PluginResponse {
  return {
    setup: (self: Interactable) => {
      self.properties.offset = { x: 0, y: 0 };
    },
    draw: (self: Interactable) => {
      if (!self.parent) return;
      self.position = {
        x: self.parent!.position.x + self.properties.offset.x,
        y: self.parent!.position.y + self.properties.offset.y,
      };
    },
  };
}
