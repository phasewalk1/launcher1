// Import necessary Electron modules.
const ipcRenderer = require("electron").ipcRenderer;

document.getElementById("loginButton")?.addEventListener("click", () => {
  // Send a message to the main process to open the Steam login window.
  ipcRenderer.send("open-steam-login");
});

ipcRenderer.on("steam-login-success", (event, steamId) => {
  console.log("Logged in with Steam ID:", steamId);
});
