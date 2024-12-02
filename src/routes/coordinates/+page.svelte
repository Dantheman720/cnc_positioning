<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    interface BitCoordinates {
        bit_id: string;
        name: string;
        x: number;
        y: number;
        z: number;
    }

    let coordinates = $state<BitCoordinates[]>([]);
    let isLoading = $state(true);
    let error = $state<string | null>(null);
    let editingId = $state<string | null>(null);

    // Load coordinates on component mount
    $effect(() => {
        loadCoordinates();
    });

    async function loadCoordinates() {
        try {
            isLoading = true;
            error = null;
            const result = await invoke<BitCoordinates[]>('get_bit_coordinates');
            coordinates = result;
        } catch (err) {
            error = err instanceof Error ? err.message : 'Failed to load coordinates';
            console.error('Error loading coordinates:', err);
        } finally {
            isLoading = false;
        }
    }

    async function saveCoordinates(coordinate: BitCoordinates) {
        try {
            error = null;
            await invoke('modify_bit_coordinates', { coordinate });
            editingId = null;
            await loadCoordinates();
        } catch (err) {
            error = err instanceof Error ? err.message : 'Failed to save coordinates';
            console.error('Error saving coordinates:', err);
        }
    }

    function startEditing(id: string) {
        editingId = id;
    }

    function cancelEditing() {
        editingId = null;
    }

    function validateCoordinate(value: string): boolean {
        const num = parseFloat(value);
        return !isNaN(num) && isFinite(num);
    }
    type Tab = 'home' | 'coordinates';
    let currentTab = $state<Tab>('home');

</script>

<div class="coordinates-manager">
    <h1>Bit Coordinates Manager</h1>
    <div class="tabs">
        <a
                class="tab-button"
                on:click={() => currentTab = 'home'}
                href="/"
        >
            Home
        </a>
    </div>


    {#if error}
        <div class="error-message">
            {error}
        </div>
    {/if}

    {#if isLoading}
        <div class="loading">Loading coordinates...</div>
    {:else}
        <div class="coordinates-table">
            <table>
                <thead>
                <tr>
                    <th>Bit Name</th>
                    <th>X</th>
                    <th>Y</th>
                    <th>Z</th>
                    <th>Actions</th>
                </tr>
                </thead>
                <tbody>
                {#each coordinates as coord}
                    <tr>
                        {#if editingId === coord.bit_id}
                            <td>{coord.name}</td>
                            <td>
                                <input
                                        type="number"
                                        step="0.0001"
                                        bind:value={coord.x}
                                        class="coordinate-input"
                                />
                            </td>
                            <td>
                                <input
                                        type="number"
                                        step="0.0001"
                                        bind:value={coord.y}
                                        class="coordinate-input"
                                />
                            </td>
                            <td>
                                <input
                                        type="number"
                                        step="0.0001"
                                        bind:value={coord.z}
                                        class="coordinate-input"
                                />
                            </td>
                            <td class="actions">
                                <button
                                        class="save-button"
                                        on:click={() => saveCoordinates(coord)}
                                >
                                    Save
                                </button>
                                <button
                                        class="cancel-button"
                                        on:click={cancelEditing}
                                >
                                    Cancel
                                </button>
                            </td>
                        {:else}
                            <td>{coord.name}</td>
                            <td>{coord.x.toFixed(4)}</td>
                            <td>{coord.y.toFixed(4)}</td>
                            <td>{coord.z.toFixed(4)}</td>
                            <td>
                                <button
                                        class="edit-button"
                                        on:click={() => startEditing(coord.bit_id)}
                                >
                                    Edit
                                </button>
                            </td>
                        {/if}
                    </tr>
                {/each}
                </tbody>
            </table>
        </div>
    {/if}
</div>

<style>
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

    .coordinates-manager {
        padding: 20px;
        max-width: 1000px;
        margin: 0 auto;
    }

    h1 {
        font-size: 24px;
        margin-bottom: 20px;
        color: #333;
    }

    .coordinates-table {
        overflow-x: auto;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        background: white;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    }

    th, td {
        padding: 12px;
        text-align: left;
        border-bottom: 1px solid #eee;
    }

    th {
        background: #f5f5f5;
        font-weight: 600;
        color: #333;
    }

    .coordinate-input {
        width: 100px;
        padding: 6px;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 14px;
    }

    .coordinate-input:focus {
        outline: none;
        border-color: #007bff;
    }

    button {
        padding: 6px 12px;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
        transition: background-color 0.2s;
    }

    .edit-button {
        background: #007bff;
        color: white;
    }

    .edit-button:hover {
        background: #0056b3;
    }

    .save-button {
        background: #28a745;
        color: white;
        margin-right: 8px;
    }

    .save-button:hover {
        background: #218838;
    }

    .cancel-button {
        background: #dc3545;
        color: white;
    }

    .cancel-button:hover {
        background: #c82333;
    }

    .error-message {
        padding: 12px;
        background: #f8d7da;
        border: 1px solid #f5c6cb;
        border-radius: 4px;
        color: #721c24;
        margin-bottom: 20px;
    }

    .loading {
        text-align: center;
        padding: 20px;
        color: #666;
    }

    .actions {
        display: flex;
        gap: 8px;
    }
</style>