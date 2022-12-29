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
		We can be contacted though email <a href="mailto:kiaora@naff.nz">kiaora@naff.nz</a>, or on
		instagram via @naff2023
	</p>
</Paragraph>

<style lang="scss">
</style>
