<script>
	import { onMount } from 'svelte';

	/**
	 * @type {HTMLTextAreaElement}
	 */
	let leftTextarea;
	/**
	 * @type {HTMLTextAreaElement}
	 */
	let rightTextarea;

	function copyToLeft() {
		rightTextarea.value = leftTextarea.value;
	}

	function copyToRight() {
		leftTextarea.value = rightTextarea.value;
	}

	onMount(() => {
		leftTextarea.focus();
	});
</script>

<div class="fixed top-0 left-0 w-full bg-gray-200 z-50">
	<div class="container mx-auto py-4">
		<div class="flex justify-center items-center">
			<h1 class="text-lg font-bold">JSON Array 与 NDJSON 互相转换</h1>
		</div>
	</div>
</div>

<div class="container mx-auto mt-32">
	<div class="flex justify-center">
		<div class="w-1/12" />
		<div class="w-4/12 px-2">
			<textarea class="w-full h-80 border border-gray-400 p-2" placeholder="JSON" bind:this={leftTextarea} />
		</div>
		<div class="w-2/12 flex justify-center items-center">
			<div class="flex flex-col justify-center">
				<button
					class="bg-indigo-500 hover:bg-indigo-600 text-white font-bold py-2 px-4 rounded-lg mb-4" on:click={copyToLeft}
					>JSON>>NDJSON</button
				>
				<button class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-lg" on:click={copyToRight}
					>NDJSON>>JSON</button
				>
			</div>
		</div>
		<div class="w-4/12 px-2">
			<textarea class="w-full h-80 border border-gray-400 p-2" placeholder="NDJSON" bind:this={rightTextarea} />
		</div>
		<div class="w-1/12" />
	</div>
</div>
