import { useEffect, useState } from "react";

// Baked at build time: vite envPrefix exposes GREETING_TAG as a literal here,
// and esbuild folds this one template literal into a single string in the
// bundle (splitting it would break greps against dist/).
const line = `frontend: hello world oxzoo-rust-react_${import.meta.env.GREETING_TAG}`;

export default function App() {
  const [backend, setBackend] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetch("/api/greeting")
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.text();
      })
      .then((text) => setBackend(text))
      .catch((err) => setError(err.message));
  }, []);

  return (
    <main style={{ fontFamily: "system-ui, sans-serif", maxWidth: "640px", margin: "4rem auto", padding: "0 1rem" }}>
      <h1>oxzoo-rust-react</h1>
      <p>{line}</p>
      {error !== null ? (
        <p>backend: error: {error}</p>
      ) : backend === null ? (
        <p>backend: loading</p>
      ) : (
        <p>backend: {backend}</p>
      )}
    </main>
  );
}
