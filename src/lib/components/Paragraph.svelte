<script lang="ts">
	import A2 from './A2.svelte';

	type Colors = 'yellow' | 'green' | 'blue' | 'pink' | 'red';
	export let color: Colors;

	let tone: 'light' | 'dark' = color == 'green' || color == 'red' ? 'light' : 'dark';
</script>

<div class="paragraph {color}">
	<A2 style={tone}><slot name="title" /></A2>
	<div class="content">
		<slot name="content" />
	</div>
</div>

<style lang="scss">
	.paragraph {
		width: 100%;
		padding: 1rem;
		box-shadow: 5px 5px 0px #000000;
		text-align: center;
		color: #222;

		.content {
			padding: 1rem;
		}

		&.yellow {
			background-color: rgb(var(--bg-yellow));
		}

		&.green {
			background-color: rgb(var(--bg-green));
		}

		&.blue {
			background-color: rgb(var(--bg-blue));
		}

		&.pink {
			background-color: rgb(var(--bg-pink));
		}

		&.red {
			background-color: rgb(var(--bg-red));
		}

		&.red,
		&.green {
			color: #fff;
		}
	}
</style>
