<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<svelte:head>
	<title>TraefikCtl</title>
</svelte:head>

{@render children()}
