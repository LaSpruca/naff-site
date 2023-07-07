<script lang="ts">
	import { page } from '$app/stores';
	let pageHref: string;
	$: pageHref = $page.url.href.replace($page.url.protocol + '//' + $page.url.host, '');
</script>

<header>
	<img src="/assets/img/naff-logo.png" alt="NAFF Logo" height="50px" />
	<nav>
		<a href="/" class:active={pageHref == '/'}>About</a>
		<a href="/rules" class:active={pageHref.startsWith('/rules')}>Rules</a>
		<a href="/participate" class:active={pageHref.startsWith('/participate')}>Participate</a>
		<a href="/contact" class:active={pageHref == '/contact'}>Contact us</a>
		<a href="https://www.eventfinda.co.nz/2023/naff-screening-2023/whangarei" target="_blank" rel="noreferrer" class:active={pageHref == '/contact'}>Tickets</a>
	</nav>
</header>

<style lang="scss">
	header {
		background-color: rgb(var(--bg-yellow));
		height: 7rem;

		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		gap: 0.5rem;

		box-shadow: 0px 4px 7px rgba(var(--bg-yellow), 0.4);
	}

	nav {
		display: flex;
		gap: 1rem;

		a {
			font-family: 'Roboto', sans-serif;
			text-decoration: none;
			color: #00000080;
			font-weight: bold;
			padding: 5px;
			transition: all ease-in-out 0.1s;
			border-radius: 1px;

			&:hover {
				color: #000000ff;
			}

			&.active {
				color: #000000ff;
				box-shadow: 0 0 0 4px #000;
			}
		}
	}
</style>
