<script>
	let aframe_loaded = false;
	export let greet;
	export let wasm;

	let app_view = 0;

	let app = wasm.new_app();

	let c_app_view_listener = wasm.app_c_current_view_listen(app, (app_view2) => {
		app_view = app_view2;
	});

	import("https://aframe.io/releases/1.0.4/aframe.min.js").then(() => aframe_loaded = true);

	function log_in() {
		wasm.app_log_in(app);
	}
</script>

<!--
<svelte:head>
	<script src="https://aframe.io/releases/1.0.4/aframe.min.js"></script>
</svelte:head>
-->

<main>
	<link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.15.0/css/all.css">

	{#if app_view == 0}
		<p>Loading...</p>
	{:else if app_view == 1}
		<button on:click={log_in}>Log In</button>
	{:else if app_view == 2}
		{#if aframe_loaded}
			<div class="icon-bar">
				<a href="#"><i class="fa fa-copy"></i></a>
				<a href="#"><i class="fa fa-paste"></i></a>
				<a href="#"><i class="fa fa-trash"></i></a>
				<a href="#"><i class="fa fa-pencil-alt"></i></a>
				<a href="#"><i class="fa fa-bezier-curve"></i></a>
				<a href="#"><i class="fa fa-expand-arrows-alt"></i></a>
				<a href="#"><i class="fa fa-ruler"></i></a>
				<a href="#"><i class="fa fa-object-group"></i></a>
				<a href="#"><i class="fa fa-object-ungroup"></i></a>
			</div>
			<a-scene embedded>
				<a-box position="-1 0.5 -3" rotation="0 45 0" color="#4CC3D9"></a-box>
				<a-sphere position="0 1.25 -5" radius="1.25" color="#EF2D5E"></a-sphere>
				<a-cylinder position="1 0.75 -3" radius="0.5" height="1.5" color="#FFC65D"></a-cylinder>
				<a-plane position="0 0 -4" rotation="-90 0 0" width="4" height="4" color="#7BC8A4"></a-plane>
				<a-sky color="#ECECEC"></a-sky>
				<a-entity line="start: 0, 1, 0; end: 2 0 -5; color: red"
				line__2="start: -2, 4, 5; end: 0 4 -3; color: green"></a-entity>
			</a-scene>
		{/if}
	{/if}
	<!-- <h1>{greet}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	-->
</main>

<style>
	a-scene {
		height: 100%;
		width: 100%;
	}

	/* :global(body) { margin: 0; padding: 8; } */

	main {
		margin: 0;
		padding: 0;
		width: 100%;
		height: 100%;
		padding: 0;
		display: flex;
		flex-direction: column;
	}
</style>
