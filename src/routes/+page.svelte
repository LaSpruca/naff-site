<script>
	import A3 from '$lib/components/A3.svelte';
	import Paragraph from '$lib/components/Paragraph.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
</script>

<svelte:head>
	<title>NAFF</title>
</svelte:head>

<SectionHeader>A filmmaking competition for those of us in Northland</SectionHeader>

<Paragraph color="blue">
	<svelte:fragment slot="title">what is naff?</svelte:fragment>
	<p slot="content">
		The Northland Amateur Filmakers Festival or NAFF for short, is all about unleashing Te Tai
		Tokerau's Creativity. NAFF is a free to enter film festival with judging from industry
		professionals
	</p>
</Paragraph>

<Paragraph color="pink">
	<svelte:fragment slot="title">how we started</svelte:fragment>
	<div slot="content">
		<p>
			The Northland Amateur Filmakers Festival was created in 2005 by Hayley Clark & Papanui
			Polamalu for no real reason but they liked making short films and knew a few others who also
			liked making them, so why not create a event to screen them? Plus the acronym made Hayley &
			Papanui giggle. There was no fee to submit (and still isnt) and every one got NAFF trophies
			designed by Hayley & Papanui. The trophys were designed to be intentionally NAFF, inspired by
			the Fanny the Wonder Dog trophies (If you know this reference your Hayleys best friend for
			life). In 2005 NAFF was being screened on VHS & DVD submissions, and by 2010 most submissions
			were being screened on USB drives. Hayley & Papanui had amazing suport from local filmakers.
		</p>
		<p class="strong">After a 12 year break NAFF is back</p>
	</div>
</Paragraph>

<Paragraph color="green">
	<svelte:fragment slot="title">wanna join?</svelte:fragment>
	<div slot="content" class="items">
		<A3 style="light">registrations open</A3>
		<p class="open-date">20th of december 2022</p>
		<div>
			<A3 style="light">interested in joining?</A3>
			<a href="/participate">sign up here</a>
		</div>
		<div>
			<A3 style="light">as always</A3>
			<a href="/rules">read the rules</a>
		</div>
	</div>
</Paragraph>

<style lang="scss">
	.strong {
		padding-top: 0.5rem;
		font-weight: bold;
		font-weight: bold;
	}

	.items {
		display: grid;
		grid-template-columns: auto;
		gap: 0.5rem;

		text-transform: uppercase;

		p {
			font-weight: bold;
		}

		div {
			padding-top: 0.5rem;

			:global(h3) {
				padding-bottom: 0.5rem;
			}

			a {
				font-weight: bold;
				font-size: 13pt;
				text-decoration: none;
				color: #fffc;
				transition: 0.1s ease-in-out all;

				&:hover {
					color: #ffff;
				}
			}
		}
	}

	@media screen and (min-width: 1000px) {
		.items {
			grid-template-columns: auto auto;
			row-gap: 1rem;
		}
	}
</style>
