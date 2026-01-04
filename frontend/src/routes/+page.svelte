<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
		// The WASM game is loaded automatically by macroquad/miniquad
		// It will render to the #glcanvas element in app.html
		console.log('Flappy Bird WASM game loaded!');
	});
</script>

<svelte:head>
	<title>Web Flappy Bird</title>
</svelte:head>

<main>
	<!-- The game canvas is in app.html and managed by macroquad -->
</main>

<style>
	main {
		width: 100%;
		height: 100vh;
		margin: 0;
		padding: 0;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
