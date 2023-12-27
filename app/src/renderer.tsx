// src/renderer.tsx
import React from "react";
import ReactDOM from "react-dom";

const Dashboard = () => {
  return (
    <div>
      <h1>Platform Integrations Dashboard</h1>
      <SteamLoginButton />
      {/* Add more integration components here */}
    </div>
  );
};

const SteamLoginButton = () => {
  const handleLogin = () => {
    if (window.electron) {
      window.electron.send("open-steam-login");
    } else {
      console.error("Electron's ipcRenderer is not available");
    }
  };
  const buttonStyle = {
    backgroundColor: "#0e6f44", // Steam green color
    color: "white",
    padding: "10px 15px",
    border: "none",
    borderRadius: "5px",
    cursor: "pointer",
    fontSize: "16px",
  };

  // Use the style in the button
  return (
    <button style={buttonStyle} onClick={handleLogin}>
      Login with Steam
    </button>
  );
};

ReactDOM.render(<Dashboard />, document.getElementById("root"));
