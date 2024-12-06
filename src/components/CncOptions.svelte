<script lang="ts">
    interface Props {
        onGenerateZero: () => void;
        onGenerateHeight: () => void;
        onMoveToSpoilboardZero: () => void;
    }

    let { onGenerateZero, onGenerateHeight, onMoveToSpoilboardZero } = $props();
</script>

<div class="actions-container">
    <button
            class="action-button"
            onclick={onGenerateZero}
    >
        Calculate Workpiece Zero
    </button>

    <button
            class="action-button"
            onclick={onGenerateHeight}
    >
        Calculate Workpiece Height
    </button>

    <button
            class="action-button"
            onclick={onMoveToSpoilboardZero}
    >
        Move to Spoilboard Zero
    </button>
</div>

<style>
    .actions-container {
        display: flex;
        flex-direction: column;
        gap: 12px;
        max-width: 200px;
    }

    .action-button {
        padding: 12px;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 14px;
        cursor: pointer;
        transition: background-color 0.2s;
        text-align: center;
    }

    .action-button:hover {
        background-color: #0056b3;
    }

    .action-button:active {
        background-color: #004085;
    }
</style>