<script lang="ts">
	import { page } from '$app/stores';
	let pageHref: string;
	$: pageHref = $page.url.href.replace($page.url.protocol + '//' + $page.url.host, '');
	$: console.log(pageHref);
</script>

<header>
	<img src="/assets/img/naff-logo.png" alt="NAFF Logo" height="50px" />
	<nav>
		<a href="/" class:active={pageHref == '/'}>About</a>
		<a href="/rules" class:active={pageHref.startsWith('/rules')}>Rules</a>
		<a href="/participate" class:active={pageHref.startsWith('/participate')}>Participate</a>
		<a href="/contact" class:active={pageHref == '/contact'}>Contact us</a>
	</nav>
</header>

<style lang="scss">
	header {
		z-index: 999;
		background-color: var(--bg-yellow);

		padding: 1rem;
		height: 7rem;
		width: 100vw;
		max-width: 100%;

		position: fixed;
		top: 0;
		left: 0;

		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 1rem;

		filter: drop-shadow(0 4px 4px #000000cc);

		nav {
			display: flex;
			align-items: center;
			justify-content: center;
			flex-direction: row;
			gap: 1rem;

			a {
				color: var(--text);
				text-decoration: none;
				font-weight: bold;
				font-size: 15px;

				&.active {
					color: var(--text-disabled);
					font-weight: bold;
				}
			}
		}
	}
</style>
