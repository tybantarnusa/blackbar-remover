<script lang="ts">
    import "bulma/css/bulma.min.css";
    import { onDestroy, onMount } from "svelte";

    let removeBlackbar: any;
    let inputImage: any;
    let selectedImage: any;
    let errMsg = $state("");
    let filename = $state("");
    let resultImage: any = $state("");
    let loading = $state(false);
    let downloadable = $state(false);

    onMount(async () => {
        const module = await import("../pkg/blackbar_remover.js");
        await module.default({ wasm: import.meta.env.WASM_PATH });
        removeBlackbar = module.remove_black_bar;
    });

    function handleImageUpload() {
        loading = false;
        downloadable = false;
        errMsg = "";

        if (resultImage) {
            URL.revokeObjectURL(resultImage);
        }

        if (!inputImage || !inputImage.files || inputImage.files.length === 0)
            return;
        selectedImage = inputImage.files[0];
        if (selectedImage) {
            const reader = new FileReader();
            reader.readAsDataURL(selectedImage);
            reader.onload = async (e) => {
                if (!e.target) return;
                resultImage = e.target.result;
                filename = selectedImage.name;
            };
        }
    }

    function handleRemoveBlackbar() {
        loading = true;
        if (inputImage.files[0]) {
            selectedImage = inputImage.files[0];
        }
        if (selectedImage) {
            const reader = new FileReader();
            reader.onload = async (e) => {
                if (!e.target) {
                    loading = false;
                    return;
                }

                try {
                    const data = new Uint8Array(e.target.result as ArrayBuffer);
                    const output = removeBlackbar(data);
                    const blob = new Blob([output], { type: "image/png" });
                    resultImage = URL.createObjectURL(blob);
                    errMsg = "";
                    downloadable = true;
                } catch (error: any) {
                    errMsg = error.message;
                }

                loading = false;
            };
            reader.readAsArrayBuffer(selectedImage);
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
                <p class="subtitle">Remove black bars from images</p>
            </div>
            {#if errMsg}
                <div
                    class="notification is-danger is-light"
                    style="padding: 15px;"
                >
                    <button
                        aria-label="delete"
                        class="delete"
                        onclick={() => {
                            errMsg = "";
                        }}
                    ></button>
                    <strong>Error:</strong> {errMsg}
                </div>
            {/if}
            {#if resultImage}
                <div class="box block has-background-grey-darker">
                    <div
                        role="button"
                        onclick={() =>
                            document.getElementById("image-input")?.click()}
                        onkeypress={() => {}}
                        tabindex="0"
                    >
                        <img
                            style="max-height: 500px;"
                            src={resultImage}
                            alt="result"
                        /><br />
                    </div>
                </div>
                {#if downloadable}
                    <div class="block is-centered">
                        <a
                            class="button is-small is-white"
                            href={resultImage}
                            download
                            ><i class="fa fa-download" aria-hidden="true"
                            ></i>&nbsp;Download</a
                        >
                    </div>
                {/if}
            {/if}
            <div class="file is-centered" class:has-name={filename}>
                <label class="file-label">
                    <input
                        id="image-input"
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
                            onclick={handleRemoveBlackbar}
                            >Remove Blackbar</button
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
        font-size: 40px;
    }

    p.subtitle {
        font-family: Calibri, sans-serif;
        font-size: 18px;
    }
</style>
