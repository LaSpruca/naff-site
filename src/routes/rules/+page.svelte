<script lang="ts">
	import Section from '$lib/components/Section.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
</script>

<svelte:head>
	<title>RULES | NAFF</title>
</svelte:head>

<SectionHeader>Competition Rules</SectionHeader>

<Section color="blue">
	<svelte:fragment slot="title">Rules and regulations</svelte:fragment>
	<div slot="content">
		<ul class="list">
			<li>Short films must be submitted by the 10th of July</li>
			<li>
				NAFF welcomes filmmakers of all ages and skill levels, regardless of their level of 
				experience, but you must submit the films in the appropriate category.
			</li>
			<li>
				There is no restriction in theme, genre, or style. However, this is a short film festival,
				and films must have a story or narrative to them. Organisers reserve the right to
				not screen your entry.
			</li>
			<li>
				NAFF reserves the right to refuse to screen films that are offensive, especially when
				directed at individuals or groups. Decisions on this will be made at the discretion of the
				NAFF committee. Films shouldn’t exceed an M rating.
			</li>
			<li>
				We will not accept or screen home movies, random footage, or edits using other movies footage 
				unless rights are accessed.
			</li>
			<li>
				Films can be anything up to, but not over 10 minutes (If its a few seconds over it will be
				accepted). This includes your titles and credits.
			</li>
			<li>Films must be uploaded via the link on website</li>
			<li>
				As all films -- unless deemed against rules and regulations -- will be screened to public, 
				please ensure that you have acquired the appropriate copyright permissions and release forms.
			</li>
			<li>
				These films will be screened in a public, family friendly setting. We WILL rate all films,
				and although we will moderate these, please remember that this is a family-friendly event.
			</li>
		</ul>
		<p>NAFF is a free to enter, not-for-profit event. Thank you to our sponsors!</p>
	</div>
</Section>

<style lang="scss">
	.list {
		list-style: none;
		padding-left: 0;
		li {
			position: relative;
			padding: 1rem;
			margin-left: 0;
			&:not(:last-child)::after {
				--clip: polygon(50% 0%, 100% 40%, 100% 60%, 50% 100%, 0 60%, 0 40%);

				content: '';
				position: absolute;
				width: 100%;
				height: 2px;
				bottom: 0;
				left: 0;
				background-color: #0005;

				-webkit-clip-path: var(--clip);
				clip-path: var(--clip);
			}
		}
	}

	.list ~ p {
		padding-top: 1.5rem;
		font-weight: bold;
	}
</style>
