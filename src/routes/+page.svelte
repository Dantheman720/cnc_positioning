<script lang="ts">
    import RouterBitSelector from '../components/RouterBitSelector.svelte';
    import PlywoodWidthInput from '../components/PlywoodWidthInput.svelte';
    import CncOptions from '../components/CncOptions.svelte';
    import {invoke} from "@tauri-apps/api/core";
    import CreateRouterBit from '../components/CreateRouterBit.svelte';
    import BitCoordinates from "../components/BitCoordinates.svelte";

    type Tab = 'home' | 'coordinates' | 'create-bit';
    let currentTab = $state<Tab>('home');
    function handleBitCreated() {
        // Optionally refresh the router bit list in RouterBitSelector
        // You might need to add a refresh method to RouterBitSelector
        currentTab = 'home';
    }

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

    let error = $state<string | null>(null);

    let selectedBit = $state<RouterBit | null>(null);
    let width = $state('');

    async function handleGenerateZero() {
        if (!selectedBit || !width) {
            error = 'Please select a router bit and enter plywood thickness';
            return;
        }

        error = null;

        try {
            await invoke('move_to_workpiece_zero', {
                data: JSON.stringify({
                    router_bit: selectedBit,
                    plywood_thickness: parseFloat(width),
                    calculate_workpiece_zero: true,
                    calculate_workpiece_height: false
                })
            });
        } catch (err) {
            error = err instanceof Error ? err.message : 'Failed to generate Z coordinate G-code';
            console.error("Failed to generate Z coordinate G-code:", err);
        }
    }

    async function handleGenerateHeight() {
        if (!selectedBit || !width) {
            error = 'Please select a router bit and enter plywood thickness';
            return;
        }

        error = null;

        try {
            await invoke('set_z_machine_coordinate', {
                data: JSON.stringify({
                    router_bit: selectedBit,
                    plywood_thickness: parseFloat(width),
                    calculate_workpiece_zero: false,
                    calculate_workpiece_height: true
                })
            });
        } catch (err) {
            error = err instanceof Error ? err.message : 'Failed to generate height G-code';
            console.error("Failed to generate height G-code:", err);
        }
    }
</script>

<main class="container">
        <h1>CNC Position Selector</h1>

        <div class="tabs">
            <a
                    class="tab-button"
                    class:active={currentTab === 'home'}
                    onclick={() => currentTab = 'home'}
                    href="#home"
            >
                Home
            </a>
            <a
                    class="tab-button"
                    class:active={currentTab === 'coordinates'}
                    onclick={() => currentTab = 'coordinates'}
                    href="#coordinates"
            >
                Bit Coordinates
            </a>
            <a
                    class="tab-button"
                    class:active={currentTab === 'create-bit'}
                    onclick={() => currentTab = 'create-bit'}
                    href="#create-bit"
            >
                Create Bit
            </a>
        </div>

        {#if currentTab === 'home'}

    <div class="setup-form">
        <div class="form-section">
            <h2>Router Bit Selection</h2>
            <RouterBitSelector
                    selectedBitAction={selectedBit}
                    onSelect={(bit) => selectedBit = bit}
            />
        </div>

        <div class="form-section">
            <h2>Plywood Settings</h2>
            <PlywoodWidthInput setWidth={(wd) => width = wd}/>
        </div>

        <div class="form-section">
            <h2>Calculation Options</h2>
            <CncOptions
                    onGenerateZero={handleGenerateZero}
                    onGenerateHeight={handleGenerateHeight}
            />
        </div>

    </div>
        {:else if currentTab === 'create-bit'}
            <CreateRouterBit onSuccess={handleBitCreated} />
        {:else}
            <BitCoordinates/>
        {/if}

    </main>

<style>
    .container {
        max-width: 800px;
        margin: 0 auto;
        padding: 20px;
    }

    .tabs {
        display: flex;
        gap: 10px;
        margin-bottom: 30px;
        border-bottom: 1px solid #eee;
        padding-bottom: 10px;
    }

    .tab-button {
        padding: 10px 20px;
        border: none;
        background: none;
        cursor: pointer;
        font-size: 16px;
        color: #666;
        border-radius: 4px;
        transition: all 0.2s ease;
    }

    .tab-button:hover {
        background: #f5f5f5;
        color: #333;
    }

    .tab-button.active {
        background: #007bff;
        color: white;
    }

    .setup-form {
        display: flex;
        flex-direction: column;
        gap: 30px;
    }

    .form-section {
        border: 1px solid #eee;
        padding: 20px;
        border-radius: 8px;
        background: white;
    }

    h2 {
        font-size: 18px;
        color: #444;
        margin-bottom: 15px;
    }
</style>