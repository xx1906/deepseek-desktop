import { useEffect } from "react";
import "./App.css";

function App() {
  useEffect(() => {
    window.location.href = "https://chat.deepseek.com";
  }, []);

  return (
    <div className="loading">
      <p>Redirecting to DeepSeek Chat...</p>
    </div>
  );
}

export default App;
