import EventEmitter from "events";
import { Vector2 } from "./math";
import { Dugtrio } from "..";

export interface PluginResponse {
  draw: (self: Interactable) => void;
  setup: (self: Interactable) => void;
}

export class Interactable extends EventEmitter {
  private interactables: Interactable[] = [];
  private plugIns: ((self: Interactable) => void)[] = [];

  properties: Record<string, any> = {};

  draw: (self: Interactable) => void = () => {};
  position: Vector2 = { x: 0, y: 0 };
  size: Vector2 = { x: 0, y: 0 };

  child(interactable: Interactable) {
    this.interactables.push(interactable);
  }

  addPlugin(plugin: PluginResponse) {
    plugin.setup(this);
    this.plugIns.push(plugin.draw);
  }

  render() {
    for (const plugin of this.plugIns) {
      plugin(this);
    }

    this.draw(this);

    for (const interactable of this.interactables) {
      interactable.render();
    }
  }
}
