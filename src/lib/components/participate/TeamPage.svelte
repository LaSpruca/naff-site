<script lang="ts">
	import { browser } from '$app/environment';
	import Paragraph from '../Paragraph.svelte';
	import Section from '../Section.svelte';
	import * as api from '$lib/client/api';
	import { createEventDispatcher } from 'svelte';

	export let team: api.Team;
	export let members: api.User[];
	export let user: api.User;

	let dispatch = createEventDispatcher<{ 'update-team': null }>();

	let filteredMembers: api.User[] = [];
	$: filteredMembers = members.filter((member) => member.id != user.id);

	let showModal = false;

	$: if (browser) {
		if (showModal) {
			document.body.style.position = 'fixed';
			document.body.scrollTo({
				top: 0,
				behavior: 'auto'
			});
		} else {
			document.body.style.position = '';
		}
	}

	let errorMsg: null | string = null;

	const leaveTeam = () => {
		api
			.leaveTeam()
			.then(() => {
				showModal = false;
				dispatch('update-team', null);
			})
			.catch((ex) => {
				console.error(ex);
				errorMsg = "Couldn't leave team\ntry reloading page or trying again";
			});
	};
</script>

<div class="modal" class:shown={showModal}>
	<Paragraph color="red">
		<svelte:fragment slot="title">Are you sure?</svelte:fragment>

		<div slot="content">
			<p>Are you sure you want to leave <span class="team-name">{team.name}</span>?</p>
			{#if errorMsg}
				<p class="error">{errorMsg}</p>
			{/if}
			<div class="leave-options">
				<button on:click={leaveTeam}>Yes</button>
				<button on:click={() => (showModal = false)}>No</button>
			</div>
		</div>
	</Paragraph>
</div>

<Section color="red">
	<svelte:fragment slot="title">Welcome {user.name}</svelte:fragment>
	<div slot="content">
		<p>You are in <span class="team-name">{team.name}</span></p>
		<p>Your team code is <span class="team-name">{team.id}</span></p>
		<button on:click={() => (showModal = true)} class="leave-button">Leave team</button>
	</div>
</Section>

<Section color="blue">
	<svelte:fragment slot="title">Film Details</svelte:fragment>
</Section>

<Section color="green">
	<svelte:fragment slot="title">Team members</svelte:fragment>
	<div slot="content" class="members">
		<div class="member-row title">
			<p>Name</p>
			<p>Email</p>
		</div>

		{#each members as member}
			<div class="member-row">
				<div>{member.name}</div>
				<div>{member.email}</div>
			</div>
		{/each}
	</div>
</Section>

<style lang="scss">
	.error {
		color: black;
	}

	.team-name {
		font-weight: bold;
	}

	.leave-button {
		margin-top: 1rem;
	}

	.members {
		display: flex;
		align-items: center;
		flex-direction: column;
	}

	.member-row {
		width: 75%;
		display: flex;
		justify-content: space-between;

		&.title {
			font-weight: bold;
		}
	}

	.modal {
		position: absolute;
		align-items: center;
		justify-content: center;
		left: 0;
		top: 0;
		width: 100vw;
		max-width: 100%;
		height: 100vh;
		z-index: 99;
		backdrop-filter: brightness(0.1);
		display: none;

		& > :global(div) {
			width: fit-content !important;
			max-width: 100%;
			height: fit-content;
			z-index: 100;
		}

		.leave-options {
			display: flex;
			align-items: center;
			justify-content: center;
			gap: 1rem;
			margin-top: 1rem;
		}

		&.shown {
			display: flex;
		}
	}
</style>
