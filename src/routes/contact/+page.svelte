<script lang="ts">
	import Paragraph from '$lib/components/Paragraph.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
</script>

<svelte:head>
	<title>CONTACT | NAFF</title>
</svelte:head>

<SectionHeader>Get in Touch</SectionHeader>
<Paragraph color="pink">
	<svelte:fragment slot="title">Contact Options</svelte:fragment>
	<p slot="content">
		We can be contacted though email: <a href="mailto:kiaora@naff.nz">kiaora@naff.nz</a>
		<br>
		Or, on Instagram: <a href="https://www.instagram.com/naff2023/">@naff2023</a>
	</p>
</Paragraph>

<style lang="scss">
	a{
		color: inherit;
		text-decoration: none;
		margin-left: 0.5rem;
	}
	a:hover{
		text-decoration: underline;
	}
</style>
