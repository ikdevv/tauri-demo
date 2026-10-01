import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export function Home() {
  const [greetMsg, setGreetMsg] = useState("");
  const [price, setPrice] = useState(0);
  const [quantity, setQuantity] = useState(0);

  async function calculateTotal() {
    setGreetMsg(await invoke("calculate_total", { price, quantity }));
  }

  return (
    <main className="min-h-screen bg-gray-950 flex items-center justify-center">
      <div className="w-full max-w-md bg-gray-900 rounded-2xl shadow-xl p-8">
        <h1 className="text-3xl font-bold text-white text-center mb-8">
          Tauri Calculator
        </h1>

        <form
          className="flex flex-col gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            calculateTotal();
          }}
        >
          <input
            type="number"
            onChange={(e) => setPrice(parseFloat(e.currentTarget.value))}
            placeholder="Enter a price..."
            className="w-full px-4 py-3 rounded-lg bg-gray-800 text-white placeholder-gray-500 border border-gray-700 focus:outline-none focus:border-blue-500 transition-colors"
          />
          <input
            type="number"
            onChange={(e) => setQuantity(parseInt(e.currentTarget.value))}
            placeholder="Enter a quantity..."
            className="w-full px-4 py-3 rounded-lg bg-gray-800 text-white placeholder-gray-500 border border-gray-700 focus:outline-none focus:border-blue-500 transition-colors"
          />
          <button
            type="submit"
            className="w-full py-3 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded-lg transition-colors cursor-pointer"
          >
            Calculate Total
          </button>
        </form>

        {greetMsg && (
          <p className="mt-6 text-center text-xl font-medium text-green-400">
            {greetMsg}
          </p>
        )}
      </div>
    </main>
  );
}
