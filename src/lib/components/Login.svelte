<script lang="ts">
	import { login } from '$lib/client/auth0';
	import { auth0Ready } from '$lib/client/stores';
	import Section from './Section.svelte';
</script>

<Section color="red">
	<svelte:fragment slot="title">Login</svelte:fragment>

	<svelte:fragment slot="content">
		<button on:click={login} disabled={$auth0Ready}>Login</button>
	</svelte:fragment>
</Section>
