<script lang="ts">
  import { onMount } from 'svelte';
  import { getVersion } from '@tauri-apps/api/app';

  async function isTauri(): Promise<boolean> {
    try {
      await getVersion();
      return true;
    } catch {
      return false;
    }
  }


  let isDrawing = false;
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;

  type Point = [number, number];

  interface Line {
    points: Point[];
    color: string;
    width: number;
  }

  let currentLine: Point[] = [];
  let lines: Line[] = [];
  let drawingColor = '#000000';
  let lineWidth = 2;

  function startDrawing(e: MouseEvent) {
    if (!ctx) return;
    isDrawing = true;
    currentLine = [[e.offsetX, e.offsetY]];
    ctx.beginPath();
    ctx.moveTo(e.offsetX, e.offsetY);
  }

  function draw(e: MouseEvent) {
    if (!isDrawing || !ctx) return;
    currentLine.push([e.offsetX, e.offsetY]);
    ctx.lineTo(e.offsetX, e.offsetY);
    ctx.stroke();
  }

  function stopDrawing() {
    if (isDrawing) {
      isDrawing = false;
      lines.push({
        points: currentLine,
        color: drawingColor,
        width: lineWidth,
      });
    }
  }

  function clearCanvas() {
    if (ctx && canvas) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      lines = [];
    }
  }

  function redrawFromLines() {
    if (!ctx) return;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (const line of lines) {
      if (line.points.length === 0) continue;

      ctx.beginPath();
      ctx.strokeStyle = line.color;
      ctx.lineWidth = line.width;

      ctx.moveTo(line.points[0][0], line.points[0][1]);
      for (const [x, y] of line.points.slice(1)) {
        ctx.lineTo(x, y);
      }
      ctx.stroke();
    }

    // restore default style
    ctx.strokeStyle = drawingColor;
    ctx.lineWidth = lineWidth;
  }

  async function saveWhiteboard() {
    if (isTauri()) {
    const { writeTextFile } = await import('@tauri-apps/plugin-fs');
    const { save } = await import('@tauri-apps/plugin-dialog');

    const file = await save({
      title: 'Save Whiteboard',
      defaultPath: 'whiteboard.json',
      filters: [{ name: 'Whiteboard', extensions: ['json'] }]
    });

    if (file) {
      await writeTextFile(file, JSON.stringify(lines, null, 2));
      alert(`Saved to ${file}`);
    }
  } else {
      // Web fallback
      const defaultName = prompt('Enter a filename', 'whiteboard.json');
      if (!defaultName) return;

      const blob = new Blob([JSON.stringify(lines, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);

      const a = document.createElement('a');
      a.href = url;
      a.download = defaultName;
      a.click();

      URL.revokeObjectURL(url);
      alert(`Saved as ${defaultName}`);
    }
  }

  async function loadWhiteboard() {
    if (isTauri()) {
      const { readTextFile } = await import('@tauri-apps/plugin-fs');
      const { open } = await import('@tauri-apps/plugin-dialog');

      const file = await open({
        multiple: false,
        filters: [{ name: 'Whiteboard', extensions: ['json'] }]
      });

      if (file && typeof file === 'string') {
        const content = await readTextFile(file);
        lines = JSON.parse(content);
        redrawFromLines();
      }
    } else {
      // Browser fallback: trigger file input
      const input = document.createElement('input');
      input.type = 'file';
      input.accept = '.json';
      input.onchange = async () => {
        if (!input.files || input.files.length === 0) return;

        const file = input.files[0];
        const text = await file.text();
        lines = JSON.parse(text);
        redrawFromLines();
      };
      input.click();
    }
  }

  function initCanvas() {
    if (!canvas) return;
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    ctx = canvas.getContext('2d');
    if (ctx) {
      ctx.lineWidth = 2;
      ctx.lineCap = 'round';
      ctx.strokeStyle = '#000000';
    }
  }

  onMount(() => {
    console.log('Running in Tauri?', isTauri());
    initCanvas();
    window.addEventListener('resize', initCanvas);
    return () => window.removeEventListener('resize', initCanvas);
  });
</script>

<main>
  <canvas
    bind:this={canvas}
    on:mousedown={startDrawing}
    on:mousemove={draw}
    on:mouseup={stopDrawing}
    on:mouseleave={stopDrawing}
    class="whiteboard"></canvas>

  <nav>
    <button on:click={clearCanvas} class="clear-button">🧹 Clear</button>
    <button on:click={saveWhiteboard}>💾 Save JSON</button>
    <button on:click={loadWhiteboard}>📂 Load JSON</button>
  </nav>
</main>

<style>
  canvas.whiteboard {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: white;
    z-index: 0;
  }

  nav {
    position: fixed;
    z-index: 1;
    top: 10px;
    left: 10px;
    display: flexbox;
    gap: 5px;
  }

  nav button {
    background: #eee;
    border: 1px solid #ccc;
    padding: 0.5rem;
    cursor: pointer;
    font-size: 1rem;
  }
</style>
