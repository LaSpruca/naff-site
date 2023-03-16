<script lang="ts">
	import Section from '../Section.svelte';
	import * as api from '$lib/client/api';
	import { createEventDispatcher } from 'svelte';

	const teamCodeRegex = /^[A-Z\d]{3}-[A-Z\d]{3}$/g;

	let mode: null | 'join' | 'create' = null;

	let dispatch = createEventDispatcher<{ 'team-update': api.Team }>();

	let teamCode = '';
	let teamCodeValid = true;
	$: teamCodeValid = teamCodeRegex.test(teamCode.toUpperCase());

	let errorMessage: string | null = null;

	$: if (!teamCodeValid) {
		errorMessage = "Your team code isn't formatted properly\nTeam codes should be XXX-XXX";
	} else {
		errorMessage = null;
	}

	$: if (teamCode.length > 3 && teamCode[3] != '-') {
		teamCode = `${teamCode.substring(0, 3)}-${teamCode.substring(3)}`;
	}

	$: teamCode = teamName.trim();

	let teamName = '';
	let teamNameValid = true;
	$: teamNameValid = teamName.trim() != '';

	$: if (!teamNameValid) {
		errorMessage = 'Please specify a team name';
	} else {
		errorMessage = null;
	}

	$: if (mode == null) {
		teamCode = '';
		teamName = '';
		errorMessage = null;
	}

	const handleTeamRequest = (req: Promise<api.Team>) => {
		req
			.then((team) => {
				dispatch('team-update', team);
			})
			.catch((ex) => {
				if (ex instanceof api.ApiError) {
					console.error(ex);
					console.error(ex.message);
					errorMessage = ex.message;
				} else {
					console.error(ex);
					errorMessage = 'There was an error talking to the server\nPlease try again later';
				}
			});
	};

	$: console.log(errorMessage);

	const joinTeam = () => handleTeamRequest(api.joinTeam(teamCode));
	const createTeam = () => handleTeamRequest(api.createTeam(teamName));
</script>

<Section color="green">
	<svelte:fragment slot="title">Join a team</svelte:fragment>

	<div slot="content">
		<p class="text">It looks like you aren't in a team yet. Lets fix that!</p>

		<div class="options-wrapper">
			<div class="options left" class:shown={mode == null}>
				<button on:click={() => (mode = 'join')}>Join a team</button>
				<p>or</p>
				<button on:click={() => (mode = 'create')}>Create a team</button>
			</div>
			<div class="options" class:shown={mode == 'join'}>
				<label for="team-code">
					Team Code
					<input
						type="text"
						id="team-code"
						placeholder="XXX-XXX"
						bind:value={teamCode}
						class:error={!teamCodeValid}
					/>
				</label>
				{#if errorMessage}
					<span class="error">
						{errorMessage}
					</span>
				{/if}
				<button on:click={joinTeam} disabled={!teamCodeValid}>Join Team</button>
				<button on:click={() => (mode = null)}>Back</button>
			</div>
			<div class="options" class:shown={mode == 'create'}>
				<label for="team-name">
					Team name
					<input
						type="text"
						id="team-name"
						placeholder="My super cool team"
						class:error={!teamNameValid}
						bind:value={teamName}
					/>
				</label>
				{#if errorMessage}
					<span class="error">
						{errorMessage}
					</span>
				{/if}
				<button on:click={createTeam} disabled={!teamNameValid}>Create team</button>
				<button on:click={() => (mode = null)}>Back</button>
			</div>
		</div>
	</div>
</Section>

<style lang="scss">
	.text {
		font-weight: light;
		font-style: italic;
		padding-bottom: 1rem;
	}

	.options-wrapper {
		position: relative;
		height: 15rem;
		overflow: hidden;
	}

	.options {
		position: absolute;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		width: 100%;
		height: 100%;
		gap: 1rem;
		opacity: 0;
		transform: translateX(100%);
		transition: all 0.2s ease-in-out;

		&.left {
			transform: translate(-100%);
		}

		&.shown {
			transform: none;
			opacity: 1;
		}

		.error {
			white-space: pre-wrap;
			color: red;
		}

		label {
			display: flex;
			align-items: flex-start;
			justify-content: center;
			flex-direction: column;
		}
	}
</style>
