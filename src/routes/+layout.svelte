<script lang="ts">
	// import type { LayoutData } from './$types';
	// export let data: LayoutData;

	import '../global.scss';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<Header />

<div class="page">
	<main>
		<slot />
	</main>

	<Footer />
</div>

<style>
	.page {
		max-width: 1400px;
		display: flex;
		min-height: 100vh;

		align-items: center;
		justify-content: space-between;

		flex-direction: column;

		margin: auto;
	}
</style>
