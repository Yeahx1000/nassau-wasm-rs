'use client'
import { useState, useEffect } from "react";

type Metrics = {
  point_of_sail: string,
  optimal_sail_angle: number,
  trim_score: number,
  speed_factor: number,
  notes: string,
}

type WasmModule = {
  calculate_metrics: (tws: number, twa: number, sail_angle: number) => Metrics
}



export default function Home() {
  const [wasm, setWasm] = useState<WasmModule | null>(null);
  const [twa, setTwa] = useState<number>(90);
  const [tws, setTws] = useState<number>(12);
  const [sailAngle, setSailAngle] = useState<number>(45);
  const [metrics, setMetrics] = useState<Metrics | null>(null);
  
  useEffect(() => {
    let alive: boolean = true;

    (async () => {
      // @ ts-ignore
      const mod = (await import("wasm-pkg")) as WasmModule;
      if (alive) setWasm(mod);
    })()

    return () => { alive = false; }
  }, [])

  useEffect(() => {
    if (!wasm) return;
    const metric = wasm.calculate_metrics(tws, twa, sailAngle);
    setMetrics(metric);
  }, [wasm, twa, tws, sailAngle])

  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <h1>Nassau WASM</h1>
    </div>
  );
}
