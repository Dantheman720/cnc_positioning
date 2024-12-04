<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";

    let onSuccess = $props();

    let error = $state<string | null>(null);
    let successMessage = $state<string | null>(null);

    let formData = $state({
        name: '',
        type: '',
        diameter: '',
    });

    async function handleSubmit(e: Event) {
        e.preventDefault();
        error = null;
        successMessage = null;

        // Basic validation
        if (!formData.name || !formData.type || !formData.diameter) {
            error = 'Please fill in all required fields';
            return;
        }

        try {
            const routerBitData = {
                name: formData.name,
                type: formData.type,
                diameter: parseFloat(formData.diameter),
                description: '',
            };

            await invoke('create_router_bit', {
                data: JSON.stringify(routerBitData)
            });

            successMessage = 'Router bit created successfully';

            // Reset form
            formData = {
                name: '',
                type: '',
                diameter: '',
            };
        } catch (err) {
            error = err instanceof Error ? err.message : 'Failed to create router bit';
            console.error("Failed to create router bit:", err);
        }
    }
</script>

<form onsubmit={handleSubmit} class="router-bit-form">
    <h2>Create New Router Bit</h2>

    {#if error}
        <div class="error">{error}</div>
    {/if}

    {#if successMessage}
        <div class="success">{successMessage}</div>
    {/if}

    <div class="form-group">
        <label for="name">Name *</label>
        <input
                type="text"
                id="name"
                bind:value={formData.name}
                required
        />
    </div>

    <div class="form-group">
        <label for="type">Type *</label>
        <input
                type="text"
                id="type"
                bind:value={formData.type}
                required
        />
    </div>

    <div class="form-group">
        <label for="diameter">Diameter (inches) *</label>
        <input
                type="number"
                id="diameter"
                step="0.001"
                bind:value={formData.diameter}
                required
        />
    </div>

    <button type="submit" class="submit-button">Create Router Bit</button>
</form>

<style>
    .router-bit-form {
        padding: 20px;
        background: white;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .form-group {
        margin-bottom: 15px;
    }

    label {
        display: block;
        margin-bottom: 5px;
        font-weight: 500;
    }

    input {
        width: 100%;
        padding: 8px;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 14px;
    }

    .error {
        color: #dc3545;
        margin-bottom: 15px;
        padding: 10px;
        background: #ffe6e6;
        border-radius: 4px;
    }

    .success {
        color: #28a745;
        margin-bottom: 15px;
        padding: 10px;
        background: #e6ffe6;
        border-radius: 4px;
    }

    .submit-button {
        background: #007bff;
        color: white;
        border: none;
        padding: 10px 20px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 16px;
        width: 100%;
    }

    .submit-button:hover {
        background: #0056b3;
    }
</style>