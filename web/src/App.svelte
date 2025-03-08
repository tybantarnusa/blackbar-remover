<script lang="ts">
    import "bulma/css/bulma.min.css";
    import { onDestroy, onMount } from "svelte";

    let removeBlackbar: any;
    let inputImage: any;
    let resultImage: any = $state("");

    onMount(async () => {
        const module = await import("../pkg/blackbar_remover.js");
        await module.default("../pkg/blackbar_remover_bg.wasm");
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
            };
        }
    }

    function handleRemoveBlackbar() {
        const image = inputImage.files[0];
        if (image) {
            const reader = new FileReader();
            reader.readAsArrayBuffer(image);
            reader.onload = async (e) => {
                const data = new Uint8Array(e.target.result);
                const output = removeBlackbar(data);
                const blob = new Blob([output], { type: "image/png" });
                resultImage = URL.createObjectURL(blob);
            };
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
            <section class="hero">
                <div class="hero-body">
                    <p class="title">Blackbar Remover</p>
                </div>
            </section>
            {#if resultImage}
                <div class="box block has-background-grey-darker">
                    <img
                        style="max-height: 500px;"
                        src={resultImage}
                        alt="Result"
                    /><br />
                </div>
            {/if}
            <div class="file has-name is-centered">
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
                    <span class="file-name">
                        Screen Shot 2017-07-29 at 15.54.25.png
                    </span>
                </label>
            </div>
            {#if resultImage}
                <div class="block">
                    <button
                        class="button is-medium is-primary is-outlined is-fullwidth"
                        onclick={handleRemoveBlackbar}>Remove Blackbar</button
                    >
                </div>
            {/if}
        </div>
    </div>
</main>

<style>
p.title {
    font-family: Calibri, sans-serif;
    font-size: 50px;
}
</style>
