<script lang="ts">
	import { PUBLIC_BACKEND } from '$env/static/public';
	import Section from '$lib/components/Section.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
	const { user, teams } = data;
</script>

<svelte:head>
	<title>PARTICIPATE | NAFF</title>
</svelte:head>

<SectionHeader>Admin Portal</SectionHeader>

<a href="{PUBLIC_BACKEND}/auth/logout">Logout</a>

{#each teams as team}
	<Section color="green">
		<svelte:fragment slot="title">{team.name}</svelte:fragment>

		<div class="team-details" slot="content">
			<p>Team code: <strong>{team.id}</strong></p>

			{#if team.film_name}
				<p>Film name: <strong>{team.film_name}</strong></p>
				<p>Film description: {team.film_descriptions}</p>
				{#if team.has_file}
					<a href="{PUBLIC_BACKEND}/api/team/{team.id}/film/download">Download file</a>
				{/if}
			{/if}
		</div>
	</Section>
{/each}

<style lang="scss">
</style>
