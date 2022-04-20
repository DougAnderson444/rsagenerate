<script>
	import { onMount } from 'svelte';
	import init, { greet } from 'wasm';
	onMount(async () => {
		await init();
	});
</script>

<button on:click={greet}> 🦀 + 🕸 Hello WebAssembly! </button>
