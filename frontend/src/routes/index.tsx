/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

import { Title } from "@solidjs/meta";

export default function Home() {
  const handleReset = async () => {
    await fetch("/api/reset", { method: "POST" });
  };

  return (
    <main>
      <Title>Home</Title>
      <h1>Home</h1>
      <button onClick={handleReset}>Reset Database</button>
    </main>
  );
}

