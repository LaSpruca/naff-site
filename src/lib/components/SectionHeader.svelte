<div class="wrapper">
	<div class="blur" />

	<img src="/assets/img/naff-logo.png" alt="NAFF logo" width="158px" />
	<h1><slot /></h1>
</div>

<style lang="scss">
	.wrapper {
		position: relative;
		display: flex;
		flex-direction: column;
		text-align: center;
		padding: 1rem;
		padding: 1rem;
		align-items: center;
		justify-content: center;
		border-radius: 3px;
		width: 500px;
		max-width: 100%;

		background: var(--naff-grad);

		h1 {
			margin-top: 1rem;
			// height: 50px;

			font-family: 'Open Sans', sans-serif;
			color: #fff;
			font-style: normal;
			font-weight: 800;
			font-size: 1.8em;
			// line-height: 25px;
			text-align: center;
		}

		.blur {
			z-index: -1;
			position: absolute;
			left: 50%;
			top: 50%;
			transform: translate(-50%, -50%);
			width: 100%;
			height: 100%;
			background: var(--naff-grad);

			filter: blur(10px);
		}
	}

	@media screen and (min-width: 1000px) {
		img {
			width: 199px;
		}
	}
</style>
