<script lang="ts">
	import { onMount } from 'svelte'

	import init, { greet, generate_address } from 'wasm'

	let address = 'TBD'
	onMount(async () => {
		console.log('Initializing...')
		await init()
		console.log('Initialized!')
		address = generate_address()
	})
</script>

<button class="bg-red-300 m-4 p-4 rounded text-lg" on:click={greet}>
	🦀 + 🕸 Hello WebAssembly!
</button>

<div class="">
	Address: {address}
</div>
