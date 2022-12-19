<script lang="ts">
	let className: string = '';
	export let color: 'red' | 'blue' = 'blue';

	export { className as class };
</script>

<button on:click class={className} class:red={color == 'red'}><slot /></button>

<style lang="scss">
	button {
		background-color: #3daee9;
		border: none;
		font-size: 15pt;
		padding: 0.35rem 1rem;
		color: white;
		border-radius: 3px;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.4);
		transition: all 0.1s ease-in-out;

		&:hover {
			transform: translateY(-5%);
			box-shadow: 0 0 10px rgba(0, 0, 0, 0.7);
		}

		&.red {
			background-color: rgb(255, 29, 40);
		}
	}
</style>
