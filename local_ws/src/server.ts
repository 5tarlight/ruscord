import { WebSocketServer } from "ws";

const wss = new WebSocketServer({ port: 8080 });

wss.on("connection", function connection(ws, req) {
  ws.on("error", console.error);

  console.log(`New Client connected ${req.socket.remoteAddress}`);
  // Send Hello(10) event
  ws.send(
    JSON.stringify({
      op: 10,
      d: {
        heartbeat_interval: 45000,
      },
    })
  );

  ws.on("message", function message(data) {
    console.log("echo: %s", data);
    ws.send(data);
  });
});

console.log("WebSocket Server is listening in port 8080.");
