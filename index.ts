import net from "net";

export class Dugtrio {
  private static connection: net.Socket;
  public static init() {
    const PIPE_NAME = "discord_ipc";
    const PIPE_PATH = `\\\\.\\pipe\\${PIPE_NAME}`;

    const server = net.createServer((socket) => {
      console.log("Dugtrio Connected");
      this.connection = socket;

      socket.on("end", () => {
        console.log("Dugtrio disconnected");
      });
    });

    server.listen(PIPE_PATH, () => {
      console.log(`Dugtrio is ready!`);
    });
  }

  public static send(message: any) {
    this.connection.write(`${message}\n`);
  }
}
