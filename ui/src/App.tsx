import { useState } from "react";
import rustbase_logo from "/rustbase_logo.png";
import { CircleCheck } from "lucide-react";
import { CircleX } from "lucide-react";
import "./App.css";

const API_URL: String = import.meta.env.VITE_BACKEND_URL;

function App() {
  const [status, setStatus] = useState<Boolean | null>(null);

  const checkStatus = async () => {
    const response = await fetch(`${API_URL}/health`);

    if (response.ok) {
      setStatus(true);
    } else {
      setStatus(false);
    }
  };

  return (
    <>
      <div>
        <a href="https://github.com/anthonyb8/rustbase" target="_blank">
          <img src={rustbase_logo} className="logo" alt="Vite logo" />
        </a>
      </div>
      <h1>Rustbase</h1>
      <p className="read-the-docs">Click on the logo to learn more</p>
      <div className="card">
        <button onClick={checkStatus}>Check Status</button>
        {status === true ? (
          <CircleCheck color="green" />
        ) : status === false ? (
          <CircleX color="red" />
        ) : null}
      </div>
    </>
  );
}

export default App;
