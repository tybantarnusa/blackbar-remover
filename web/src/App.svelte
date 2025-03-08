<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    let removeBlackbar: any;
    let inputImage: any;
    let resultImage: any = $state("");

    onMount(async () => {
        const module = await import ('../pkg/blackbar_remover.js')
        await module.default('../pkg/blackbar_remover_bg.wasm')
        removeBlackbar = module.remove_black_bar
    })

    function handleImageUpload() {
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
                const blob = new Blob([output], { type: 'image/png' });
                resultImage = URL.createObjectURL(blob);
            }
        }
    }

    onDestroy(() => {
    if (resultImage) {
      URL.revokeObjectURL(resultImage);
    }
  });
</script>

<main>
  <h1>Blackbar Remover</h1>
  <p>* Baru bisa ngapus bar hitam kiri-kanan, atas-bawah masih dalam roadmap</p>
  <img width="640" src={resultImage} alt="Result" style="padding-bottom: 20px;" /><br />
  <input type="file" accept="image/*" onchange={handleImageUpload} bind:this={inputImage} style="padding-bottom: 20px;" />
  <button onclick={handleRemoveBlackbar}>Remove Blackbar</button><br />
</main>
