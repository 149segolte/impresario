<script lang="ts">
	import General from '../lib/General.svelte';
</script>

<div class="h-full flex flex-row text-white text-sm">
	<div id="sidebar" class="w-64 h-full flex flex-col overflow-y-auto">
		<div class="m-1 rounded-md flex flex-row items-center">
			<img class="w-8 m-[10px]" src="/favicon.png" alt="logo" />
			<h1 class="text-lg font-bold">Impresario</h1>
		</div>
		<div class="active m-1 rounded-md flex flex-row items-center hover:bg-neutral-800">
			<img class="w-4 m-[10px]" src="https://via.placeholder.com/16x16" alt="placeholder" />
			General
		</div>
		<div class="m-1 rounded-md flex flex-row items-center hover:bg-neutral-800">
			<img class="w-4 m-[10px]" src="https://via.placeholder.com/16x16" alt="placeholder" />
			Associations
		</div>
		<div class="m-1 rounded-md flex flex-row items-center hover:bg-neutral-800">
			<img class="w-4 m-[10px]" src="https://via.placeholder.com/16x16" alt="placeholder" />
			Profiles
		</div>
		<div class="m-1 mt-auto rounded-md flex flex-row items-center hover:bg-neutral-800">
			<img class="w-4 m-[10px]" src="https://via.placeholder.com/16x16" alt="placeholder" />
			Guide to Impresario
		</div>
		<div class="m-1 rounded-md flex flex-row items-center hover:bg-neutral-800">
			<img class="w-4 m-[10px]" src="https://via.placeholder.com/16x16" alt="placeholder" />
			Give Feedback
		</div>
	</div>
	<div class="w-full m-4">
		<General />
	</div>
</div>

<style lang="postcss">
	.active {
		@apply bg-neutral-800;
	}
</style>
