import net from "net";
import {
  CircleCommand,
  ColorCommand,
  LineCommand,
  Payload,
  RectCommand,
  RoundingCommand,
  ThicknessCommand,
} from "./src/drawlist";

export class DrawingContext {
  static rect(payload: Omit<RectCommand, "type">) {
    Dugtrio.currentFrame.commands.push({ ...payload, type: "rect" });
  }
  static circle(payload: Omit<CircleCommand, "type">) {
    Dugtrio.currentFrame.commands.push({ ...payload, type: "circle" });
  }
  static color(payload: Omit<ColorCommand, "type">) {
    Dugtrio.currentFrame.commands.push({ ...payload, type: "color" });
  }
  static line(payload: Omit<LineCommand, "type">) {
    Dugtrio.currentFrame.commands.push({ ...payload, type: "line" });
  }
  static thickness(payload: Omit<ThicknessCommand, "type">) {
    Dugtrio.currentFrame.commands.push({ ...payload, type: "thickness" });
  }
  static rounding(payload: Omit<RoundingCommand, "type">) {
    Dugtrio.currentFrame.commands.push({ ...payload, type: "rounding" });
  }
}

export class Dugtrio {
  private static connection: net.Socket | null = null;
  static currentFrame: Payload = { commands: [] };

  public static init() {
    const PIPE_NAME = "discord_ipc";
    const PIPE_PATH = `\\\\.\\pipe\\${PIPE_NAME}`;

    const server = net.createServer((socket) => {
      console.log("Dugtrio Connected");
      this.connection = socket;

      socket.on("end", () => {
        this.connection = null;
        console.log("Dugtrio disconnected");
      });
    });

    server.listen(PIPE_PATH, () => {
      console.log(`Dugtrio is ready!`);
    });
  }

  public static draw() {
    this.send(this.currentFrame);
    this.clear();
  }

  private static clear() {
    this.currentFrame = { commands: [] };
  }

  private static send(message: Payload) {
    if (!this.connection) return;
    this.connection.write(`${JSON.stringify(message)}\n`);
  }
}
