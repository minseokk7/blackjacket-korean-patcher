<script lang="ts">
  import "../app.css";
  let { children } = $props();
</script>

<div class="h-screen w-screen bg-gradient-to-br from-[#07080c] via-[#0d1017] to-[#120a14] text-slate-100 flex flex-col font-sans select-none overflow-hidden">
  {@render children()}
</div>
