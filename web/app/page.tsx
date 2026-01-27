'use client'

import { useState, useEffect } from "react";

// TODO: clean up the Ui, looks awful, and organize components

type Metrics = {
  point_of_sail: string,
  optimal_sail_angle: number,
  trim_score: number,
  speed_factor: number,
  notes: string,
}

type WasmModule = {
  compute_metrics: (tws: number, twa: number, sail_angle: number) => Metrics
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
      console.log('WASM module:', mod);
      if (alive) setWasm(mod as unknown as WasmModule);
    })()

    return () => { alive = false; }
  }, [])

  useEffect(() => {
    if (!wasm) return;
    const metric = wasm.compute_metrics(tws, twa, sailAngle);
    setMetrics(metric);
  }, [wasm, twa, tws, sailAngle])

  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <h1>Nassau WASM</h1>

      {!wasm && (
        <p>Loading WASM module...</p>
      )}

      <section>
        <Card title="Inputs">
          <Slider 
            label="True Wind Speed (TWS)" 
            value={tws}
            min={0} 
            max={35}
            step={1}
            onChange={setTws}
          />
          <Slider 
            label="True Wind Angle (TWA)" 
            value={twa}
            min={0} 
            max={180}
            step={1}
            onChange={setTwa}
          />
          <Slider 
            label="Sail Angle" 
            value={sailAngle}
            min={0} 
            max={90}
            step={1}
            onChange={setSailAngle}
          />
        </Card>

        <Card title="Outputs">
          <Row label="Point of Sail" value={metrics?.point_of_sail ?? '-'} />
          <Row label="Optimal Sail Angle" value={metrics ? metrics.optimal_sail_angle.toFixed(1) + '°' : '-'} />
          <Row label="Trim Score" value={metrics ? `${metrics.trim_score}/100` : '-'} />
          <Row label="Speed Factor" value={metrics ? metrics.speed_factor.toFixed(3) : '-'} />
          <div className="mt-4 text-zinc-900 dark:text-zinc-100">
            <strong>Notes:</strong>
            <p>{metrics?.notes ?? '-'}</p>
          </div>
        </Card>
      </section>
    </div>
  );
}

function Card({ title, children }: { title: string, children: React.ReactNode }) {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md dark:bg-zinc-800">
      <h2 className="text-xl font-semibold mb-4 text-zinc-900 dark:text-zinc-100">{title}</h2>
      {children}
    </div>
  );
}

function Slider({ label, value, min, max, step, onChange }: { 
  label: string, 
  value: number,
  min: number, 
  max: number, 
  step: number, 
  onChange: (value: number) => void; 
}) {
  return (
    <div className="mb-4">
      <label className="block text-zinc-900 dark:text-zinc-100 mb-2">{label}</label>
      <input 
        type="range" 
        min={min} 
        max={max} 
        step={step} 
        value={value}
        onChange={(e) => onChange(Number(e.target.value))} 
        className="w-full"
      />
    </div>
  )
}

function Row({ label, value }: { label: string, value: string }) {
  return (
    <div className="flex justify-between mb-2">
      <span className="text-zinc-900 dark:text-zinc-100">{label}</span>
      <span className="font-mono text-zinc-900 dark:text-zinc-100">{value}</span>
    </div>
  )
}
