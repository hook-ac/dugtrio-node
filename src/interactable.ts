import { Vector2 } from "./math";

export class Interactable {
  private interactables: Interactable[] = [];
  private plugIns: ((self: Interactable) => void)[] = [];

  draw: (self: Interactable) => void = () => {};
  position: Vector2 = { x: 0, y: 0 };
  size: Vector2 = { x: 0, y: 0 };

  child(interactable: Interactable) {
    this.interactables.push(interactable);
  }

  addPlugin(plugin: (self: Interactable) => void) {
    this.plugIns.push(plugin);
  }

  render() {
    this.draw(this);

    for (const plugin of this.plugIns) {
      plugin(this);
    }

    for (const interactable of this.interactables) {
      interactable.render();
    }
  }
}
