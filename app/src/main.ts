import { app, BrowserWindow, ipcMain } from "electron";
import * as path from "path";
import axios from "axios";

function createWindow() {
  // Create the browser window.
  const mainWindow = new BrowserWindow({
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
        if (url.startsWith("your-custom-protocol://")) {
          console.log("Received URL:", url);
          const urlObj = new URL(url);
          const steamId = urlObj.searchParams.get("steamId");

          if (steamId) {
            mainWindow.webContents.send("steam-login-success", steamId);
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
  createWindow();

  app.on("activate", function () {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});
