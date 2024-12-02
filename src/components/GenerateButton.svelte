<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    interface RouterBit {
        name: string;
        type: string;
        diameter: number;
        // Add other properties as needed
    }

    let { selectedBit, width, calculateZero, calculateHeight } = $props();
    let isLoading = $state(false);
    let error = $state<string | null>(null);

    $effect(() => {
        if (selectedBit && width) {
            error = null;
        }
    });

    async function handleGenerate() {
        if (!selectedBit || !width) {
            error = 'Please select a router bit and enter plywood thickness';
            return;
        }

        const data = {
            router_bit: selectedBit,
            plywood_thickness: parseFloat(width),
            calculate_workpiece_zero: calculateZero,
            calculate_workpiece_height: calculateHeight
        };

        try {
            isLoading = true;
            error = null;
            const name = width

            // Using Tauri's invoke instead of fetch
            const content = JSON.stringify(data)
            const result = await invoke('write_positioning_file', {data: content});
            console.log('Generation successful:', result);

        } catch (err) {
            error = err instanceof Error ? err.message : 'An unknown error occurred';
            console.error('Generation error:', err);
        } finally {
            isLoading = false;
        }
    }

</script>

<div class="generate-container">
    <div class="debug-info">
        <pre>Selected Bit: {selectedBit ? selectedBit.name : 'none'}</pre>
        <pre>Width: {width || 'none'}</pre>
        <pre>Calculate Zero: {calculateZero}</pre>
        <pre>Calculate Height: {calculateHeight}</pre>
    </div>

    <button
            class="generate-button"
            on:click={handleGenerate}
            disabled={isLoading}
    >
        {#if isLoading}
            Generating...
        {:else}
            Generate
        {/if}
    </button>

    {#if error}
        <div class="error-message">
            {error}
        </div>
    {/if}
</div>

<style>
    .generate-container {
        margin: 20px 0;
        max-width: 200px;
    }

    .debug-info {
        font-family: monospace;
        font-size: 12px;
        background: #f5f5f5;
        padding: 8px;
        margin-bottom: 10px;
        border-radius: 4px;
    }

    .debug-info pre {
        margin: 0;
        white-space: pre-wrap;
        word-break: break-all;
    }

    .generate-button {
        width: 100%;
        padding: 12px 24px;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 16px;
        font-weight: 500;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .generate-button:hover:not(:disabled) {
        background-color: #0056b3;
    }

    .generate-button:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }

    .error-message {
        margin-top: 8px;
        color: #dc3545;
        font-size: 14px;
    }
</style>