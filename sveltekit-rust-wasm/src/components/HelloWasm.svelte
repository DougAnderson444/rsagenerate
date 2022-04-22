<script lang="ts">
	import { onMount } from 'svelte'

	import init, { greet, generate_address } from 'wasm'

	let address = 'Computing...'

	onMount(async () => {
		console.log('Initializing...')
		await init()
		console.log('Generating...')
		let start = Date.now()
		// todo: put this in a worker so it doesn't block
		address = generate_address('some seed value')
		address += ` (${(Date.now() - start) / 1000}) secs`
		console.log('Generated!')
	})
</script>

<button class="bg-red-300 m-4 p-4 rounded text-lg" on:click={greet}>
	🦀 + 🕸 Hello WebAssembly!
</button>

<div class="">
	Address: {address}
</div>
