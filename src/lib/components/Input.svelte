<script lang="ts">
	export let value = '';
	// export let changed = false;
	export let error: undefined | string = undefined;
	export let id: string;

	$: console.log(value);
</script>

<div class="input" class:filled={value != ''} class:error={typeof error !== 'undefined'}>
	<label for={id}><slot /></label>
	<div class="">
		<input type="text" bind:value {id} />
		{#if error}
			<p><span>Error:</span> {error}</p>
		{/if}
	</div>
</div>

<style lang="scss">
	.input {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		align-items: flex-start;
		font-weight: bold;

		input {
			border-radius: 2px;
			box-shadow: 0 0 10px 1px #000a;
			border: none;
			transition: 0.1s all ease-in-out;
			padding: 0.5rem 0.5rem;

			&:focus-within {
				box-shadow: 0 0 10px 3px #000a;
				outline: none;
			}
		}

		p {
			padding-top: 0.25rem;
			font-size: 10pt;
			span {
				font-weight: bolder;
			}
		}

		&.error {
			input {
				border: 1px solid red;
				color: red;
			}
		}
	}
</style>
