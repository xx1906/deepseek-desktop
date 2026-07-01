import { useEffect } from "react";
import { motion } from "motion/react";
import "./App.css";

function App() {
  useEffect(() => {
    window.location.href = "https://chat.deepseek.com";
  }, []);

  return (
    <motion.div
      className="loading"
      initial={{ opacity: 0, scale: 0.95 }}
      animate={{ opacity: 0.7, scale: 1 }}
      transition={{ duration: 0.6, ease: "easeOut" }}
    >
      <motion.p
        animate={{
          opacity: [0.4, 1, 0.4],
        }}
        transition={{
          duration: 1.8,
          repeat: Infinity,
          ease: "easeInOut",
        }}
      >
        Redirecting to DeepSeek Chat...
      </motion.p>
    </motion.div>
  );
}

export default App;
