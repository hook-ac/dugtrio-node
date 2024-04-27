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
import { Vector2 } from "./src/math";

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

export interface WindowData {
  mouseDown: [boolean, boolean, boolean, boolean, boolean];
  mousePosition: [number, number];
  windowSize: [number, number];
}

export class Dugtrio {
  private static connection: net.Socket | null = null;
  private static windowData: WindowData;
  static currentFrame: Payload = { commands: [] };

  public static init() {
    // SEND PIPE
    const PIPE_NAME = "discord_ipc";
    const PIPE_PATH = `\\\\.\\pipe\\${PIPE_NAME}`;

    const server = net.createServer((socket) => {
      console.log("Dugtrio Connected");
      this.connection = socket;
      socket.on("end", () => {
        this.connection = null;
        console.log("Dugtrio disconnected");
      });
      socket.on("error", (err: any) => {
        if (err.code === "EPIPE") {
          console.error("Write failed, client disconnected.");
        } else {
          console.error("Socket error:", err);
        }
      });
    });

    server.listen(PIPE_PATH, () => {
      console.log(`Dugtrio sender is ready!`);
    });

    // RECV PIPE
    const PIPE_NAME_OUT = "discord_ipc_out";
    const PIPE_PATH_OUT = `\\\\.\\pipe\\${PIPE_NAME_OUT}`;

    const server_out = net.createServer((socket) => {
      console.log("Dugtrio RECV Connected");
      socket.on("data", (data) => {
        try {
          this.windowData = JSON.parse(data.toString("utf-8").trim());
        } catch (error) {}
      });
      socket.on("end", () => {
        console.log("Dugtrio receiver disconnected");
      });
      socket.on("error", (err: any) => {
        if (err.code === "EPIPE") {
          console.error("Write failed, client disconnected.");
        } else {
          console.error("Socket error:", err);
        }
      });
    });

    server_out.listen(PIPE_PATH_OUT, () => {
      console.log(`Dugtrio receiver is ready!`);
    });
  }

  public static getCursorPosition(): Vector2 {
    if (!this.windowData) {
      return { x: 0, y: 0 };
    }
    return {
      x: this.windowData.mousePosition[0],
      y: this.windowData.mousePosition[1],
    };
  }

  public static getWindowSize(): Vector2 {
    if (!this.windowData) {
      return { x: 0, y: 0 };
    }
    return {
      x: this.windowData.windowSize[0],
      y: this.windowData.windowSize[1],
    };
  }

  public static isMouseDown(button: number): boolean {
    if (!this.windowData) {
      return false;
    }
    return this.windowData.mouseDown[button];
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
