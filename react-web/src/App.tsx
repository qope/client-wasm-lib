import React from "react";
import logo from "./logo.svg";
import init, {
  prove_simple_signature,
  prove_user_transaction,
  echo,
} from "wasm-client";
import "./App.css";

function App() {
  const userTransaction = async () => {
    init();
    console.log("start proving");
    const res = await fetch("./data/user_transaction_input.json");
    const input = JSON.stringify(await res.json());
    const proof = prove_user_transaction(input);
    console.log(proof);
  };
  return (
    <div className="App">
      <button onClick={userTransaction}> prove user transaction once</button>
    </div>
  );
}

export default App;
