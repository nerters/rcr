<script setup lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMounted} from "vue";

  onMounted(async () => { 
    await invoke("open_docs");
  })

</script>

<template>
  <div style="flex-direction: column; height: 100vh; display: flex;">
    <router-view></router-view>
  </div>

</template>

<style>
html,
body,
#app {
    height: 100%;
    margin: 0;
    padding: 0;
    scrollbar-width: none;
}



/* 滚动条宽度 */
::-webkit-scrollbar {
  width: 1px;
  height: 1px;
}
</style>