<template>
    <v-layout class="rounded rounded-md">
      <!-- Navigation Drawer -->
      <v-navigation-drawer
        :width="drawerWidth"
        floating
        permanent
      >
      <v-treeview
        :items="cache_link"
        :opened="initiallyOpen"
        item-value="title"
        activatable
        open-on-click
      >
        <template v-slot:prepend="{ item, isOpen }">
          <v-icon v-if="!item.file">
            {{ isOpen ? 'mdi-folder-open' : 'mdi-folder' }}
          </v-icon>
          <v-icon v-else>
            {{ files[item.file] }}
          </v-icon>
        </template>
      </v-treeview>
      </v-navigation-drawer>
  
  
      <v-divider :thickness="5" vertical class="resizer" :style="{ 'padding-left': drawerWidth + 'px' }"
      @mousedown="onMouseDown"></v-divider>
  
      <!-- Main Content -->
      <v-main class="d-flex align-center justify-center" style="padding-left: 0px;">
        Main 111
      </v-main>
    </v-layout>
  </template>
  
<script lang="ts" setup>
import { onBeforeMount, Ref, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { message, ask } from "@tauri-apps/plugin-dialog";
import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";


  const drawerWidth = ref(200);
  
  const cacheFileName = "redis_links.json";
  const cache_link:Ref<Array<any>> = ref([]);
  const initiallyOpen = ref(['public'])
    
  onBeforeMount(async () => {
    let data = await getCacheFile()
    if (data) {
        // 转成json字符串并格式化
        cache_link.value = [{
      title: '.git',
    }];
    } else {
        cache_link.value = [{
      title: '.git',
    }];
    }
    console.log(cache_link.value);
    await invoke("pubsub")
})

  // 鼠标按下时触发的事件
  const onMouseDown = (e: MouseEvent) => {
      const startX = e.clientX;
      const startWidth = drawerWidth.value;

      // 鼠标移动事件处理
      const onMouseMove = (moveEvent: MouseEvent) => {
      const offsetX = moveEvent.clientX - startX;
      // 限制宽度在 100px 到 (窗口宽度 - 100) 之间
      drawerWidth.value = Math.max(100, Math.min(window.innerWidth - 100, startWidth + offsetX));

      };

      // 鼠标释放时移除事件监听
      const onMouseUp = () => {
      document.removeEventListener('mousemove', onMouseMove);
      document.removeEventListener('mouseup', onMouseUp);
      };

      document.addEventListener('mousemove', onMouseMove);
      document.addEventListener('mouseup', onMouseUp);
  };




  const getCacheFile = async () : Promise<string> => {
    console.log("获取文件")
      //判断默认文件是否存在
    const exit = await exists(cacheFileName, { baseDir: BaseDirectory.AppData });
    console.log(exit)
    if (!exit) {
        console.log("创建文件")
        await create(cacheFileName, { baseDir: BaseDirectory.AppData })
    }
    let data = await readTextFile(cacheFileName, { baseDir: BaseDirectory.AppData })
    if (data) {
        // 转成json字符串并格式化
        return data;
    }
    return "";
  }


  const saveCacheFile = async (data: string) => {
    await writeTextFile(cacheFileName, data, { baseDir: BaseDirectory.AppData }) 
  }



  </script>
  



  <style scoped>
  .resizer {
    display: flex;
    cursor: ew-resize;
    height: 100vh;
  }
  </style>
  