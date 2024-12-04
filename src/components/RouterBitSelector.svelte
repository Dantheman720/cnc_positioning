<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    interface RouterBit {
        id: string;
        name: string;
        type: string;
        diameter: number;
        description: string;
        material: string;
        flutes: number;
        speedRange: string;
        feedRate: string;
        applicationTypes: string[];
    }

    interface Props {
        onSelect: (bit: RouterBit | null) => void;
        selectedBitAction: RouterBit | null;
    }

    let {onSelect, selectedBitAction} = $props();

    let searchTerm = $state('');
    let viewingBit = $state<RouterBit | null>(null);
    let showOverlay = $state(false);
    let routerBits = $state<RouterBit[]>([]);
    let loading = $state(false);
    let error = $state<string | null>(null);

    async function fetchRouterBits() {
        loading = true;
        error = null;
        try {
            const bits = await invoke<RouterBit[]>('get_router_bits');
            routerBits = bits.map(bit => ({
                ...bit,
                speedRange: bit.speedRange,
                feedRate: bit.feedRate,
                applicationTypes: bit.applicationTypes,
            }));
        } catch (err) {
            error = err instanceof Error ? err.message : 'Failed to fetch router bits';
            console.error("Failed to fetch router bits:", err);
        } finally {
            loading = false;
        }
    }

    $effect(() => {
        fetchRouterBits();
    });

    let filteredBits = $derived(routerBits.filter(bit =>
        bit.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        bit.type.toLowerCase().includes(searchTerm.toLowerCase()) ||
        bit.description.toLowerCase().includes(searchTerm.toLowerCase())
    ));

    function openSearch() {
        showOverlay = true;
        searchTerm = '';
        viewingBit = null;
    }

    function closeOverlay() {
        showOverlay = false;
        searchTerm = '';
        viewingBit = null;
    }

    function viewBitDetails(bit: RouterBit) {
        viewingBit = bit;
    }

    function backToList() {
        viewingBit = null;
    }

    function selectCurrentBit() {
        onSelect(viewingBit);
        closeOverlay();
    }
</script>

<!-- Rest of your template remains the same -->
<div class="router-bit-selector">
    <button class="select-button" on:click={openSearch}>
        {selectedBitAction ? selectedBitAction.name : 'Select Router Bit'}
    </button>

    {#if showOverlay}
        <div class="overlay" on:click={closeOverlay}>
            <div class="overlay-content" on:click|stopPropagation>
                {#if !viewingBit}
                    <div class="overlay-header">
                        <input
                                type="text"
                                bind:value={searchTerm}
                                placeholder="Search router bits..."
                                class="search-input"
                                autofocus
                        />
                        <button class="close-button" on:click={closeOverlay}>×</button>
                    </div>

                    <div class="results-container">
                        {#if loading}
                            <div class="loading">Loading router bits...</div>
                        {:else if error}
                            <div class="error">
                                {error}
                                <button on:click={fetchRouterBits}>Retry</button>
                            </div>
                        {:else if filteredBits.length === 0}
                            <div class="no-results">No router bits found</div>
                        {:else}
                            {#each filteredBits as bit}
                                <div
                                        class="bit-item"
                                        on:click={() => viewBitDetails(bit)}
                                >
                                    <h3>{bit.name}</h3>
                                    <p class="bit-type">{bit.type}</p>
                                    <div class="bit-specs">
                                        <span>Ø{bit.diameter}"</span>
                                        <span>•</span>
                                        <span>{bit.flutes} flutes</span>
                                        <span>•</span>
                                        <span>{bit.material}</span>
                                    </div>
                                </div>
                            {/each}
                        {/if}
                    </div>
                {:else}
                    <div class="details-view">
                        <div class="details-header">
                            <button class="back-button" on:click={backToList}>←</button>
                            <h2>{viewingBit.name}</h2>
                            <button class="close-button" on:click={closeOverlay}>×</button>
                        </div>

                        <div class="details-content">
                            <div class="specs-grid">
                                <div class="spec-item">
                                    <span class="label">Type</span>
                                    <span>{viewingBit.type}</span>
                                </div>
                                <div class="spec-item">
                                    <span class="label">Diameter</span>
                                    <span>{viewingBit.diameter}"</span>
                                </div>
                                <div class="spec-item">
                                    <span class="label">Material</span>
                                    <span>{viewingBit.material}</span>
                                </div>
                                <div class="spec-item">
                                    <span class="label">Speed Range</span>
                                    <span>{viewingBit.speedRange}</span>
                                </div>
                                <div class="spec-item">
                                    <span class="label">Feed Rate</span>
                                    <span>{viewingBit.feedRate}</span>
                                </div>
                            </div>

                            <div class="applications">
                                <h3>Applications</h3>
                                <div class="application-tags">
                                    {#each viewingBit.applicationTypes as app}
                                        <span class="tag">{app}</span>
                                    {/each}
                                </div>
                            </div>

                            <p class="description">{viewingBit.description}</p>

                            <button
                                    class="select-bit-button"
                                    on:click={selectCurrentBit}
                            >
                                Select This Bit
                            </button>
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</div>



<style>
    .router-bit-selector {
        position: relative;
    }

    .select-button {
        width: 100%;
        padding: 10px 15px;
        font-size: 16px;
        border: 1px solid #ccc;
        border-radius: 4px;
        background: white;
        text-align: left;
        cursor: pointer;
    }

    .select-button:hover {
        border-color: #999;
    }

    .overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .overlay-content {
        background: white;
        width: 90%;
        max-width: 600px;
        max-height: 80vh;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        display: flex;
        flex-direction: column;
    }

    .overlay-header {
        display: flex;
        gap: 10px;
        padding: 15px;
        border-bottom: 1px solid #eee;
    }

    .search-input {
        flex: 1;
        padding: 10px;
        font-size: 16px;
        border: 1px solid #ddd;
        border-radius: 4px;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 24px;
        cursor: pointer;
        padding: 0 10px;
        color: #666;
    }

    .results-container {
        overflow-y: auto;
        padding: 15px;
        max-height: calc(80vh - 80px);
    }

    .bit-item {
        padding: 15px;
        border: 1px solid #eee;
        border-radius: 4px;
        margin-bottom: 10px;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .bit-item:hover {
        background-color: #f5f5f5;
        border-color: #ddd;
    }

    .bit-specs {
        display: flex;
        gap: 8px;
        align-items: center;
        font-size: 14px;
        color: #444;
    }

    .details-view {
        height: 100%;
    }

    .details-header {
        display: flex;
        align-items: center;
        padding: 15px;
        border-bottom: 1px solid #eee;
    }

    .back-button {
        font-size: 20px;
        background: none;
        border: none;
        cursor: pointer;
        padding: 0 10px;
        color: #666;
    }

    .details-content {
        padding: 20px;
        overflow-y: auto;
    }

    .specs-grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 15px;
        margin-bottom: 20px;
    }

    .spec-item {
        display: flex;
        justify-content: space-between;
        padding: 5px 0;
        border-bottom: 1px solid #eee;
    }

    .applications {
        margin-top: 20px;
    }

    .application-tags {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        margin-top: 10px;
    }

    .tag {
        background: #f0f0f0;
        padding: 4px 8px;
        border-radius: 4px;
        font-size: 14px;
    }

    .select-bit-button {
        width: 100%;
        padding: 12px;
        background: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 16px;
        cursor: pointer;
        margin-top: 20px;
    }

    .select-bit-button:hover {
        background: #0056b3;
    }
</style>