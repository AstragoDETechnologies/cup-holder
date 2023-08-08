<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  async function open_cd_drives() {
    await invoke("open_cd_drives");
  }

  async function close_cd_drives() {
    await invoke("close_cd_drives");
  }
</script>

<main class="min-h-screen bg-[#c0c0c0] select-none">
  <h1 class="text-2xl text-center font-semibold pt-5 cs">Cup Holder PRO</h1>
  <div class="flex justify-center mt-5">
    <div class=" w-96">
      <div class="flex justify-center gap-20">
        <button on:click={open_cd_drives} class="px-2">Deploy cup holder</button
        >
        <button on:click={close_cd_drives} class="px-2"
          >Retract cup holder</button
        >
      </div>
    </div>
  </div>
</main>

<style scoped>
  .cs {
    font-family: "Comic Sans MS";
  }
</style>
