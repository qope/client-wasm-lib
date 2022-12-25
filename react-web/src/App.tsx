import React, { useState } from "react";
import "./App.css";
import { wrap } from "comlink";

function App() {
  const worker = new Worker(new URL("./wasm-worker", import.meta.url), {
    name: "wasm-worker",
    type: "module",
  });
  const workerApi = wrap<import("./wasm-worker").WasmWorker>(worker);

  const [isLoading, setIsLoading] = useState(false);
  const [proofResult, setProofResult] = useState("");
  const [duration, setDuration] = useState(0);
  const [numThreads, setNumThreads] = useState(1);

  const userTransaction = () => {
    (async () => {
      setIsLoading(true);
      setProofResult("");
      setDuration(0);
      const startTime = Date.now();
      console.log("start proving");
      const res = await fetch("./data/user_transaction_input.json");
      const data = await res.json();
      const input = JSON.stringify(data);
      const proof = await workerApi.proveTx(input, numThreads);
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
      const res = await fetch("./data/simple_signature_input.json");
      const data = await res.json();
      const input = JSON.stringify(data);
      const proof = await workerApi.proveSign(input, numThreads);
      setProofResult(proof);
      setIsLoading(false);
      setDuration(Date.now() - startTime);
    })();
  };

  return (
    <div className="App">
      <p>crossOriginIsolated: {crossOriginIsolated.toString()}</p>
      <p>
        Number of threads{" "}
        <select onChange={(e) => setNumThreads(Number(e.target.value))}>
          {Array.from(Array(navigator.hardwareConcurrency).keys()).map(
            (_, i) => (
              <option
                key={i + 1}
                value={i + 1}
                selected={i + 1 === navigator.hardwareConcurrency}
              >
                {i + 1}
              </option>
            )
          )}
        </select>
        / {navigator.hardwareConcurrency}
      </p>
      <p>
        <button onClick={simpleSignature} disabled={isLoading}>
          prove signature
        </button>
      </p>
      <p>
        <button onClick={userTransaction} disabled={isLoading}>
          prove transaction
        </button>
      </p>
      <p>Status: {!isLoading ? <>idling</> : <>proving...</>} </p>
      <p>Time: {duration} ms</p>
      <textarea value={proofResult} className="result"></textarea>
    </div>
  );
}

export default App;
