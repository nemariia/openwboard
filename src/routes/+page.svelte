<script lang="ts">
  import { onMount } from 'svelte';

  let isDrawing = false;
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D | null = null;

  function startDrawing(e: MouseEvent) {
    if (!ctx) return;
    isDrawing = true;
    ctx.beginPath();
    ctx.moveTo(e.offsetX, e.offsetY);
  }

  function draw(e: MouseEvent) {
    if (!isDrawing || !ctx) return;
    ctx.lineTo(e.offsetX, e.offsetY);
    ctx.stroke();
  }

  function stopDrawing() {
    isDrawing = false;
  }

  function clearCanvas() {
    if (ctx && canvas) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
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

  <button on:click={clearCanvas} class="clear-button">🧹 Clear</button>
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

  .clear-button {
    position: fixed;
    top: 10px;
    left: 10px;
    z-index: 1;
    background: #eee;
    border: 1px solid #ccc;
    padding: 0.5rem;
    cursor: pointer;
    font-size: 1rem;
  }
</style>
