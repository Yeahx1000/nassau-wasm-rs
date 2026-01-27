declare module 'wasm-pkg' {
  export function compute_metrics(
    tws: number,
    twa: number,
    sail_angle: number
  ): {
    point_of_sail: string;
    optimal_sail_angle: number;
    trim_score: number;
    speed_factor: number;
    notes: string;
  };
}