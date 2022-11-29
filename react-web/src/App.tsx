import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import init, {
  proveSimpleSignature,
  proveUserTransaction,
  echo,
} from "wasm-client";
import "./App.css";

function App() {
  const [isLoading, setIsLoading] = useState(false);
  const [proofResult, setProofResult] = useState("");
  const [duration, setDuration] = useState(0);

  const userTransaction = () => {
    setIsLoading(true);
    setProofResult("");
    setDuration(0);
    const startTime = Date.now();
    init().then(() => {
      console.log("start proving");
      fetch("./data/user_transaction_input.json").then((res) => {
        res.json().then((input_json) => {
          const input = JSON.stringify(input_json);
          proveUserTransaction(input).then((proof) => {
            setProofResult(proof);
            setIsLoading(false);
            setDuration(Date.now() - startTime);
          });
        });
      });
    });
  };

  const simpleSignature = () => {
    setIsLoading(true);
    setProofResult("");
    setDuration(0);
    const startTime = Date.now();
    init().then(() => {
      console.log("start proving");
      fetch("./data/simple_signature_input.json").then((res) => {
        res.json().then((input_json) => {
          const input = JSON.stringify(input_json);
          proveSimpleSignature(input).then((proof) => {
            setProofResult(proof);
            setIsLoading(false);
            setDuration(Date.now() - startTime);
          });
        });
      });
    });
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
