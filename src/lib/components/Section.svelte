<script lang="ts">
	import A2 from './A2.svelte';

	type Colors = 'green' | 'blue' | 'red';
	export let color: Colors;
	let className: string = '';
	export { className as class };

	let tone: 'light' | 'dark' = color == 'blue' ? 'dark' : 'light';
</script>

<div class="paragraph {color} {className}">
	<A2 style={tone}><slot name="title" /></A2>
	<div class="content">
		<slot name="content" />
	</div>
</div>

<style lang="scss">
	.paragraph {
		width: 100%;
		padding: 1rem;
		text-align: center;
		color: #222;
		border-left: #000 solid 15px;
		border-radius: 2px 3px 3px 2px;

		.content {
			padding: 1rem;
		}

		&.green {
			background-color: rgb(var(--bg-green));
			border-color: rgb(var(--bg-green-dark));
		}

		&.blue {
			background-color: rgb(var(--bg-blue));
			border-color: rgb(var(--bg-blue-dark));
		}

		&.red {
			background-color: rgb(var(--bg-red));
			border-color: rgb(var(--bg-red-dark));
		}

		&.red,
		&.green {
			color: #fff;
		}
	}
</style>
