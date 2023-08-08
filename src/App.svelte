<script lang="ts">
  import TitleBar from "./lib/TitleBar.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  async function open_cd_drives() {
    invoke("open_cd_drives");
  }

  async function close_cd_drives() {
    invoke("close_cd_drives");
  }
</script>

<main class="min-h-screen bg-[#c0c0c0]">
  <TitleBar />
  <h1 class="text-2xl text-center font-semibold mt-5">Cup Holder</h1>
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

<style>
</style>
