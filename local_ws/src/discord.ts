import { WebSocket } from "ws";
import dotenv from "dotenv";

dotenv.config();

// You should fetch this URL!
const ws = new WebSocket("wss://gateway.discord.gg");
const token = process.env.token;

let hb_interval = -1;
let last_seq = -1;
let first_ack_received = false;
let hb_loop: NodeJS.Timer;

const get_hb = () =>
  JSON.stringify({
    op: 1,
    d: last_seq === -1 ? null : last_seq,
  });

ws.on("error", console.error);

ws.on("open", () => {
  ws.on("message", (data) => {
    const json = JSON.parse(data.toString());
    let msg = "";

    if (json.op) {
      if (json.op === 10) {
        // Hello
        msg = "Hello Event (op 10)";
        hb_interval = json.d.heartbeat_interval;
        const jitter = Math.random() / 4;

        setTimeout(() => {
          ws.send(get_hb());
        }, hb_interval * jitter);
      } else if (json.op === 11) {
        // Heartbeat ACK
        msg = "Heartbeat ACK (op 11)";

        if (!first_ack_received) {
          first_ack_received = true;
          hb_loop = setInterval(() => {
            ws.send(get_hb());
          }, hb_interval);

          // After getting first ACK, try identify
          console.log(`Trying identifying with token ${token?.slice(0, 5)}...`);
          ws.send(
            JSON.stringify({
              op: 2,
              d: {
                token,
                intents: 513,
                properties: {
                  os: "macos",
                  browser: "ruscord",
                  device: "ruscord",
                },
              },
            })
          );
        }
      } else if (json.op === 0) {
        msg = "Dispatch (op 0)";
      }
    }

    json.type = msg;
    console.dir(json);
  });
});
