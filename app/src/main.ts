import { app, BrowserWindow, ipcMain, protocol } from "electron";
import * as path from "path";
import axios from "axios";

const steamProtocol = "steam-auth-protocol";
app.setAsDefaultProtocolClient(steamProtocol);

let mainWindow: BrowserWindow | null = null;

function createWindow() {
  // Create the browser window.
  mainWindow = new BrowserWindow({
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
      nodeIntegration: true,
      contextIsolation: false,
    },
    width: 800,
  });

  // and load the index.html of the app.
  mainWindow.loadFile(path.join(__dirname, "../index.html"));

  // Open the DevTools.
  mainWindow.webContents.openDevTools();

  ipcMain.on("open-steam-login", async () => {
    try {
      const response = await axios.get("http://127.0.0.1:8080/steam/login");
      const steamLoginUrl = response.data;

      let authWindow = new BrowserWindow({
        width: 800,
        height: 600,
        show: true, // Set to true to make the window visible
        webPreferences: {
          nodeIntegration: true,
          contextIsolation: false,
          preload: path.join(__dirname, "preload.js"),
        },
      });

      authWindow.loadURL(steamLoginUrl);
      authWindow.show();

      authWindow.webContents.on("will-navigate", (event, url) => {
        if (url.startsWith("steam-auth-protocol://")) {
          console.log("Received URL:", url);
          const urlObj = new URL(url);
          const steamId = urlObj.searchParams.get("steamId");

          if (steamId) {
            mainWindow?.webContents.send("steam-login-success", steamId);
          }
          authWindow.close();
        }
      });
    } catch (error) {
      console.error("Failed to fetch Steam login URL:", error);
    }
  });
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
  protocol.handle(steamProtocol, (request) => {
    const url = new URL(request.url);
    const steamId = url.searchParams.get("steamId");

    if (steamId) {
      mainWindow?.webContents.send("steam-login-success", steamId);
    }

    return new Response("Login Successful", {
      headers: { "Content-Type": "text/plain" },
    });
  });

  createWindow();
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
  mainWindow = null;
});
