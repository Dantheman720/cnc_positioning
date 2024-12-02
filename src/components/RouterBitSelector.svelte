<script lang="ts">
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
        selectedBitAction: RouterBit | null;  // Add selectedBit back as a prop

    }

    let {onSelect, selectedBitAction} = $props<Props>();

    let searchTerm = $state('');
    let viewingBit = $state<RouterBit | null>(null);
    let showOverlay = $state(false);

    const routerBits: RouterBit[] = [
        {
            id: "550e8400-e29b-41d4-a716-446655440000",
            name: "Straight Bit 1/4\"",
            type: "Straight",
            diameter: 0.25,
            material: "Carbide",
            flutes: 2,
            speedRange: "16000-24000 RPM",
            feedRate: "100-150 IPM",
            applicationTypes: ["Slots", "Dadoes", "Edge trimming"],
            description: "General purpose straight cutting bit"
        },
        {
            id: "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
            name: "V-Groove 60°",
            type: "V-Groove",
            diameter: 0.5,
            material: "Carbide",
            flutes: 2,
            speedRange: "16000-22000 RPM",
            feedRate: "80-120 IPM",
            applicationTypes: ["V-carving", "Chamfering", "Decorative edges"],
            description: "For V-carving and chamfering"
        },
        {
            id: "550e8400-e29b-41d4-a716-446655440001",
            name: "Ball Nose 1/8\"",
            type: "Ball Nose",
            diameter: 0.125,
            material: "Carbide",
            flutes: 2,
            speedRange: "18000-24000 RPM",
            feedRate: "60-100 IPM",
            applicationTypes: ["3D carving", "Surfacing", "Rounded corners"],
            description: "For 3D carving and surfacing"
        }, {
            id: "7f2c4a1b-8d5e-4c3f-9f6a-1d2b3e4f5a6b",
            name: "Downcut Spiral 3/8\"",
            type: "Downcut Spiral",
            diameter: 0.375,
            material: "Carbide",
            flutes: 2,
            speedRange: "16000-22000 RPM",
            feedRate: "100-150 IPM",
            applicationTypes: ["Clean top cuts", "Plywood", "Sheet goods", "No tearout cutting"],
            description: "Downcut spiral for clean top surface and reduced tearout"
        },
        {
            id: "9e8d7c6b-5a4f-3e2d-1c0b-9a8b7c6d5e4f",
            name: "Compression 3/8\"",
            type: "Compression",
            diameter: 0.375,
            material: "Carbide",
            flutes: 2,
            speedRange: "16000-22000 RPM",
            feedRate: "100-150 IPM",
            applicationTypes: ["Clean top and bottom cuts", "Sheet goods", "Melamine", "Plywood"],
            description: "Compression spiral for clean cuts on both top and bottom surfaces"
        }
    ];

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

<div class="router-bit-selector">
    <button class="select-button" on:click={openSearch}>
        {selectedBitAction ? selectedBitAction.name : 'Select Router Bit'}
    </button>

    {#if showOverlay}
        <div class="overlay" on:click={closeOverlay}>
            <div class="overlay-content" on:click|stopPropagation>
                {#if !viewingBit}
                    <!-- Search List View -->
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
                        {#if filteredBits.length === 0}
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
                    <!-- Details View -->
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