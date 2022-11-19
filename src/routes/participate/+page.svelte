<script lang="ts">
	import { loggedIn } from '$lib/client/stores';
	import Login from '$lib/components/Login.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import User from '$lib/components/User.svelte';
</script>

<svelte:head>
	<title>PARTICIPATE | NAFF</title>
</svelte:head>

<SectionHeader>Participants Portal</SectionHeader>
{#if !$loggedIn}
	<Login />
{:else}
	<User />
{/if}

<style lang="scss">
</style>
