import { Dugtrio } from "..";
import { Interactable, PluginResponse } from "../src/Interactable";

export function onClick(data: {
  onPress: (self: Interactable) => void;
  onRelease: (self: Interactable) => void;
}): PluginResponse {
  return {
    setup: (self: Interactable) => {
      if (!self.properties.pluginMouseOver) {
        throw "onClick requires mouseOver.";
      }
      self.properties.lastPressing = false;
    },
    draw: (self: Interactable) => {
      const isOver = self.properties.mouseOver;
      const lastPressing = self.properties.lastPressing;
      const isPressing = Dugtrio.isMouseDown(0);

      if (isOver && lastPressing !== isPressing) {
        self.properties.lastPressing = isPressing;
        if (isPressing) {
          data.onPress(self);
        } else {
          data.onRelease(self);
        }
      }
    },
  };
}
