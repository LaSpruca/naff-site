<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;

	let page = 'select';
</script>

<div class="red">
	<h1 class="title">Sign up for the event</h1>
	<p>Before you sign up, make sure to check out <a href="/rules">our rules</a></p>

	<div class="signups">
		<div class="screen left" class:hidden={page != 'select'}>
			<h2>I want to</h2>

			<button on:click={() => (page = 'create')}>Create a team</button>
			<p>or</p>
			<button on:click={() => (page = 'join')}>Join a team</button>
		</div>
		<div class="screen" class:hidden={page != 'create'}>
			<h2>Create a team</h2>
			<label for="team-name">Team name</label><input type="text" id="team-name" />
			<button on:click={() => (page = 'select')}>Back</button>
		</div>
		<div class="screen" class:hidden={page != 'join'}>
			<h2>Join a team</h2>
			<label for="team-id">Team ID</label><input type="text" id="team-ID" />
			<button on:click={() => (page = 'select')}>Back</button>
		</div>
	</div>
</div>

<style lang="scss">
	a {
		font-weight: bold;
		text-decoration: none;
		color: inherit;
	}

	.signups {
		overflow: hidden;
		height: 15rem;
		position: relative;
	}

	.screen {
		position: absolute;
		left: 50%;
		padding-top: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		gap: 0.5rem;
		transition: 0.3s ease-out;
		transition-delay: 0.1s;
		height: 15rem;
		opacity: 1;
		transform: translateX(-50%);

		&.hidden {
			pointer-events: none;
			transform: translateX(calc(50%));
			opacity: 0;

			transition-delay: 0.1s;

			&.left {
				transform: translateX(-150%);
			}
		}

		button {
			padding: 0.4rem;
			outline: none !important;
			border: black 2px solid;
			border-radius: 3px;

			color: var(--text-disabled);
			background-color: #000a;
			font-size: 10pt;
		}
	}
</style>
