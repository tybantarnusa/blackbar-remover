<script lang="ts">
    import "bulma/css/bulma.min.css";
    import { onDestroy, onMount } from "svelte";

    let removeBlackbar: any;
    let inputImage: any;
    let filename = $state("");
    let resultImage: any = $state("");
    let loading = $state(false);

    onMount(async () => {
        const module = await import("../pkg/blackbar_remover.js");
        await module.default({ wasm: "../pkg/blackbar_remover_bg.wasm" });
        removeBlackbar = module.remove_black_bar;
    });

    function handleImageUpload() {
        if (resultImage) {
            URL.revokeObjectURL(resultImage);
        }

        const image = inputImage.files[0];
        if (image) {
            const reader = new FileReader();
            reader.readAsDataURL(image);
            reader.onload = async (e) => {
                resultImage = e.target.result;
                filename = image.name;
            };
        }
    }

    function handleRemoveBlackbar() {
        loading = true;
        const image = inputImage.files[0];
        if (image) {
            const reader = new FileReader();
            reader.onload = async (e) => {
                const data = new Uint8Array(e.target.result);
                const output = removeBlackbar(data);
                const blob = new Blob([output], { type: "image/png" });
                resultImage = URL.createObjectURL(blob);
                loading = false;
            };
            reader.readAsArrayBuffer(image);
        } else {
            loading = false;
        }
    }

    onDestroy(() => {
        if (resultImage) {
            URL.revokeObjectURL(resultImage);
        }
    });
</script>

<main>
    <div class="container columns has-text-centered">
        <div class="column">
            <div class="block" style="margin-top: 30px; margin-bottom: 30px;">
                <p class="title">Blackbar Remover</p>
            </div>
            {#if resultImage}
                <div class="box block has-background-grey-darker">
                    <img
                        style="max-height: 500px;"
                        src={resultImage}
                        alt="Result"
                    /><br />
                </div>
            {/if}
            <div class="file is-centered" class:has-name={filename}>
                <label class="file-label">
                    <input
                        class="file-input"
                        type="file"
                        accept="image/*"
                        onchange={handleImageUpload}
                        bind:this={inputImage}
                    />
                    <span class="file-cta">
                        <span class="file-icon">
                            <i class="fas fa-upload"></i>
                        </span>
                        <span class="file-label"> Choose an image… </span>
                    </span>
                    {#if filename}
                        <span class="file-name">
                            {filename}
                        </span>
                    {/if}
                </label>
            </div>
            {#if resultImage}
                {#if loading}
                    <div class="block">
                        <p>Processing...</p>
                    </div>
                {:else}
                <div class="block">
                    <button
                        class="button is-medium is-primary is-outlined is-fullwidth"
                        onclick={handleRemoveBlackbar}>Remove Blackbar</button
                    >
                </div>
                {/if}
            {/if}
        </div>
    </div>
</main>

<style>
    p.title {
        font-family: Calibri, sans-serif;
        font-size: 45px;
    }
</style>
