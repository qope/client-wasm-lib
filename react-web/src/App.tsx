import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import init, {
  proveSimpleSignature,
  proveUserTransaction,
  initThreadPool,
  echo,
} from "wasm-client";
import "./App.css";

function App() {
  const [isLoading, setIsLoading] = useState(false);
  const [proofResult, setProofResult] = useState("");
  const [duration, setDuration] = useState(0);

  const userTransaction = () => {
    (async () => {
      setIsLoading(true);
      setProofResult("");
      setDuration(0);
      const startTime = Date.now();
      console.log("start proving");
      await init();
      // await initThreadPool(1);
      const res = await fetch("./data/user_transaction_input.json");
      const data = await res.json();
      const input = JSON.stringify(data);
      const proof = await proveUserTransaction(input);
      setProofResult(proof);
      setIsLoading(false);
      setDuration(Date.now() - startTime);
    })();
  };

  const simpleSignature = () => {
    (async () => {
      setIsLoading(true);
      setProofResult("");
      setDuration(0);
      const startTime = Date.now();
      console.log("start proving");
      await init();
      // await initThreadPool(1);
      const res = await fetch("./data/simple_signature_input.json");
      const data = await res.json();
      const input = JSON.stringify(data);
      const proof = await proveSimpleSignature(input);
      setProofResult(proof);
      setIsLoading(false);
      setDuration(Date.now() - startTime);
    })();
  };

  return (
    <div className="App">
      <p>
        <button onClick={userTransaction} disabled={isLoading}>
          prove user transaction
        </button>
      </p>
      <p>
        <button onClick={simpleSignature} disabled={isLoading}>
          prove simple signature
        </button>
      </p>
      <p>Status: {!isLoading ? <>idling</> : <>proving...</>} </p>
      {duration !== 0 ?? <p>Duration: {duration}</p>}
      <textarea value={proofResult} className="result"></textarea>
    </div>
  );
}

export default App;
