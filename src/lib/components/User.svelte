<script lang="ts">
	import Section from './Section.svelte';
	import { user } from '$lib/client/stores';
</script>

{#if $user}
	<Section color="red">
		<svelte:fragment slot="title">
			Hello {$user.name}
		</svelte:fragment>
	</Section>
{/if}
