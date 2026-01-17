import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [lastImage, setLastImage] = useState<string | null>(null);
  const [imageCount, setImageCount] = useState(0);

  useEffect(() => {
    let unlistenImage: (() => void) | null = null;
    let unlistenMonitoring: (() => void) | null = null;

    // Check initial monitoring status
    invoke<boolean>("is_monitoring")
      .then(setIsMonitoring)
      .catch(console.error);

    // Setup event listeners
    const setupListeners = async () => {
      try {
        // Listen for clipboard image detection
        unlistenImage = await listen<string>("clipboard-image-detected", (event) => {
          console.log("New image detected!", event);
          console.log("Image data length:", event.payload?.length);
          setLastImage(event.payload);
          setImageCount(prev => prev + 1);
        });
        console.log("Image listener setup complete");

        // Listen for monitoring status changes from tray
        unlistenMonitoring = await listen<boolean>("monitoring-changed", (event) => {
          console.log("Monitoring status changed:", event.payload);
          setIsMonitoring(event.payload);
        });
        console.log("Monitoring listener setup complete");
        
      } catch (error) {
        console.error("Failed to setup listeners:", error);
      }
    };

    setupListeners();

    return () => {
      if (unlistenImage) unlistenImage();
      if (unlistenMonitoring) unlistenMonitoring();
    };
  }, []);

  const toggleMonitoring = async () => {
    try {
      if (isMonitoring) {
        await invoke("stop_monitoring");
        setIsMonitoring(false);
      } else {
        await invoke("start_monitoring");
        setIsMonitoring(true);
      }
    } catch (error) {
      console.error("Failed to toggle monitoring:", error);
    }
  };

  return (
    <main className="container">
      <h1>Shotpipe - Clipboard Monitor</h1>
      
      <div style={{ marginTop: "2rem" }}>
        <h2>Monitoring Status</h2>
        <p>Status: {isMonitoring ? "🟢 Monitoring" : "🔴 Stopped"}</p>
        <button onClick={toggleMonitoring}>
          {isMonitoring ? "Stop Monitoring" : "Start Monitoring"}
        </button>
      </div>

      <div style={{ marginTop: "2rem" }}>
        <h2>Statistics</h2>
        <p>Images detected: {imageCount}</p>
      </div>

      {lastImage && (
        <div style={{ marginTop: "2rem" }}>
          <h2>Last Captured Image</h2>
          <img 
            src={`data:image/png;base64,${lastImage}`} 
            alt="Last captured" 
            style={{ maxWidth: "100%", height: "auto", border: "1px solid #ccc" }}
          />
        </div>
      )}

      <div style={{ marginTop: "2rem", padding: "1rem", background: "#f0f0f0", borderRadius: "5px" }}>
        <p><strong>How to test:</strong></p>
        <ol style={{ textAlign: "left" }}>
          <li>Click "Start Monitoring" button above</li>
          <li>Take a screenshot (Cmd+Shift+4 on Mac)</li>
          <li>The screenshot should appear below automatically</li>
        </ol>
      </div>
    </main>
  );
}

export default App;