<script lang="ts">
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import type { PageServerData } from './$types';
	import JoinTeam from '$lib/components/participate/JoinTeam.svelte';
	import TeamPage from '$lib/components/participate/TeamPage.svelte';
	import * as api from '$lib/client/api';

	export let data: PageServerData;

	let { user, team, members } = data;

	$: if (team) {
		api
			.getMembers(team.id)
			.then((val) => (members = val))
			.catch((ex) => console.error(ex));
	}
</script>

<svelte:head>
	<title>PARTICIPATE | NAFF</title>
</svelte:head>

<SectionHeader>Participants Portal</SectionHeader>

{#if team && members}
	<TeamPage on:update-team={(event) => (team = event.detail)} {user} {team} {members} />
{:else}
	<JoinTeam on:team-update={(event) => (team = event.detail)} />
{/if}

<style lang="scss">
</style>
