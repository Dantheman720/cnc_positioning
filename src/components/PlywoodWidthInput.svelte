<script>
    let {setWidth} = $props()
    let width = $state()

    function validateInput(event) {
        // Remove any non-numeric characters except decimal point
        let value = event.target.value.replace(/[^\d.]/g, '');

        // Ensure only one decimal point
        const parts = value.split('.');
        if (parts.length > 2) {
            value = parts[0] + '.' + parts.slice(1).join('');
        }

        // Update the width with the cleaned value
        width = value;
        setWidth(width)
    }
</script>

<div class="plywood-input-container">
    <label for="plywood-width">Plywood Thickness</label>
    <div class="input-group">
        <input
                type="text"
                id="plywood-width"
                value={width}
                on:input={validateInput}
                placeholder="0.750"
        />
        <span class="unit">in</span>
    </div>
    {#if width && isNaN(parseFloat(width))}
        <span class="error">Please enter a valid number</span>
    {/if}
</div>

<style>
    .plywood-input-container {
        width: 100%;
        max-width: 200px;
        margin: 10px 0;
    }

    label {
        display: block;
        margin-bottom: 5px;
        font-size: 14px;
        color: #333;
        font-weight: 500;
    }

    .input-group {
        display: flex;
        align-items: center;
        gap: 5px;
    }

    input {
        flex: 1;
        padding: 8px 12px;
        border: 1px solid #ccc;
        border-radius: 4px;
        font-size: 16px;
    }

    input:focus {
        outline: none;
        border-color: #007bff;
    }

    .unit {
        font-size: 14px;
        color: #666;
        padding: 0 5px;
    }

    .error {
        color: #dc3545;
        font-size: 12px;
        margin-top: 4px;
        display: block;
    }
</style>