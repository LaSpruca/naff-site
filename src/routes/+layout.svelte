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
		min-height: calc(100vh - 7rem);

		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-between;
	}

	main {
		width: 90vw;
		margin: 0 auto;
		padding: 1rem 0;
		display: flex;
		gap: 1rem;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		max-width: 700px;
	}
</style>
