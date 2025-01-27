<template>
    <v-layout class="rounded rounded-md">
      <!-- Navigation Drawer -->
      <v-navigation-drawer
        :width="drawerWidth"
        floating
        permanent
      >

      <div class="image-uploader" :class="{ 'hovered': isHovered }" @mouseover="handleMouseOver" @mouseleave="handleMouseLeave" @click="addListFun">
          <div class="add-image">
          <span>+</span>
          </div>
      </div>

      <v-expansion-panels>
        <v-expansion-panel v-for="(link, i) in cache_link">
          <v-expansion-panel-title v-slot="{ expanded }" @click="fetchUsers(link)" class="bg-primary position-sticky top-0 pa-3 mt-2" style="z-index: 20;">
            <v-row no-gutters>
              <v-col class="d-flex justify-start" cols="8">
                {{ link.name }}
              </v-col>
              <v-col
                class="text--secondary"
                cols="4"
              >
                <v-fade-transition leave-absolute>
                  <span
                    v-if="expanded"
                    key="0"
                    class="text-overline"
                  >
                    {{  }}
                  </span>
                  <div key="1" style="margin-top: -4px;">
                    <v-icon @click.stop="upLinkFun(link)" icon="mdi-cog" />
                    <v-icon @click.stop="delLinkFun(link)" icon="mdi-trash-can" />
                  </div>
                 
               
                </v-fade-transition>
              </v-col>
            </v-row>
          </v-expansion-panel-title>
          <v-expansion-panel-text>
            

            <v-treeview
              v-model:activated="active"
              v-model:opened="initiallyOpen"
              :items="link.children"
              :load-children="fetchUsers"
              density="compact"
              activatable
              open-on-click
              item-title="name"
              item-value="json"
              @update:activated="handleNodeClick"
              style="margin-left: -35px; "
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

          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
      </v-navigation-drawer>
  
  
      <v-divider :thickness="8" vertical class="resizer" :style="{ 'padding-left': drawerWidth + 'px' }" @mousedown="onMouseDown"></v-divider>
  
      <!-- Main Content -->
      <v-main class="d-flex align-center justify-center" style="padding-left: 0px;">
        <text>{{ content }}</text>
      </v-main>
    </v-layout>


    <v-overlay v-model="overlay" style="display:flex;justify-content: center; align-items:center; ">

      <v-card v-if="formType === 'addr'" class="mx-auto" style="width: 350px; height: 520px; overflow-y: auto;">
        <v-form >
          <v-container>
            <v-row>
              <v-col
                cols="12"
                md="12"
              >
              <v-text-field
                hide-details="auto"
                label="链接名称"
                v-model:model-value="fromData.linkName"
              ></v-text-field>
              </v-col>
            </v-row>

            <v-row>
              <v-col
                cols="12"
                md="8"
              >
                <v-text-field
                  hide-details="auto"
                  label="ip地址"
                  v-model:model-value="fromData.host"
                ></v-text-field>
              </v-col>

              <v-col
                cols="12"
                md="4"
              >
                <v-text-field
                  hide-details="auto"
                  label="端口"
                  v-model:model-value="fromData.port"
                ></v-text-field>
              </v-col>
            </v-row>


            <v-row>
              <v-col
                cols="12"
                md="12"
              >
              <v-text-field
                  hide-details="auto"
                  label="密码"
                  v-model:model-value="fromData.password"
                ></v-text-field>
              </v-col>
            </v-row>

            <v-row>
              <v-col
                cols="12"
                md="12"
              >
              <v-switch label="监听" inset v-model:model-value="fromData.listenRedis"></v-switch>
              </v-col>
            </v-row>

            <v-row>
              <v-col cols="12" md="3" sm="6">
                <v-btn rounded="0" size="x-large" block @click="addCancel">取消</v-btn>
              </v-col>
              <v-col cols="12" md="3" sm="6">
                <v-btn rounded="0" size="x-large" block @click="addLinkSubmit">保存</v-btn>
              </v-col>
            </v-row>

          </v-container>
        </v-form>
      </v-card>



      <v-card v-if="formType === 'upAddr'" class="mx-auto" style="width: 350px; height: 520px; overflow-y: auto;">
        <v-form >
          <v-container>
            <v-row>
              <v-col
                cols="12"
                md="12"
              >
              <v-text-field
                hide-details="auto"
                label="链接名称"
                v-model:model-value="fromDataRef.linkName"
              ></v-text-field>
              </v-col>
            </v-row>

            <v-row>
              <v-col
                cols="12"
                md="8"
              >
                <v-text-field
                  hide-details="auto"
                  label="ip地址"
                  v-model:model-value="fromDataRef.host"
                ></v-text-field>
              </v-col>

              <v-col
                cols="12"
                md="4"
              >
                <v-text-field
                  hide-details="auto"
                  label="端口"
                  v-model:model-value="fromDataRef.port"
                ></v-text-field>
              </v-col>
            </v-row>


            <v-row>
              <v-col
                cols="12"
                md="12"
              >
              <v-text-field
                  hide-details="auto"
                  label="密码"
                  v-model:model-value="fromDataRef.password"
                ></v-text-field>
              </v-col>
            </v-row>

            <v-row>
              <v-col
                cols="12"
                md="12"
              >
              <v-switch label="监听" inset v-model:model-value="fromDataRef.listenRedis"></v-switch>
              </v-col>
            </v-row>

            <v-row>
              <v-col cols="12" md="3" sm="6">
                <v-btn rounded="0" size="x-large" block @click="upCancel">取消</v-btn>
              </v-col>
              <v-col cols="12" md="3" sm="6">
                <v-btn rounded="0" size="x-large" block @click="upLinkSubmit">保存</v-btn>
              </v-col>
            </v-row>

          </v-container>
        </v-form>
      </v-card>
    </v-overlay>
  </template>
  
<script lang="ts" setup>
import { onBeforeMount, reactive, Ref, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { message, ask } from "@tauri-apps/plugin-dialog";
import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";

  const drawerWidth = ref(230);
  
  const cacheFileName = "redis_links.json";
  const cache_link:Ref<Array<any>> = ref([]);
  const initiallyOpen = ref(['public'])

  const active = ref([]);
  const content = ref("");
  //遮罩使用
  const overlay = ref(false);
  //表单类型
  const formType = ref("");
  //添加地址
  const fromData = reactive({
    id: "",
    linkName: "",
    host: "",
    port: "",
    password: "",
    username: "",
    linkType: "",
    listenRedis: false,
    key: "",
    level: 0,
    db: "",
  })
  //清理
  function clearFormData() {
    fromData.id = "";
    fromData.linkName = "";
    fromData.host = "";
    fromData.port = "";
    fromData.password = "";
    fromData.username = "";
    fromData.linkType = "";
    fromData.listenRedis = false;
    fromData.key = "";
    fromData.level = 0;
  }

    //添加地址
  const fromDataRef = reactive({
    id: "",
    linkName: "",
    host: "",
    port: "",
    password: "",
    username: "",
    linkType: "",
    listenRedis: false,
    key: "",
    level: 0,
    db: "",
  })
  //清理
  function clearFormDataRef() {
    fromDataRef.id = "";
    fromDataRef.linkName = "";
    fromDataRef.host = "";
    fromDataRef.port = "";
    fromDataRef.password = "";
    fromDataRef.username = "";
    fromDataRef.linkType = "";
    fromDataRef.listenRedis = false;
    fromDataRef.key = "";
    fromDataRef.level = 0;
  }


  onBeforeMount(async () => {
    let data = await getCacheFile()
    if (data) {
        // 转成json字符串并格式化
        cache_link.value = JSON.parse(data);
        for (let i in cache_link.value) {
          cache_link.value[i].children = [];
          if (!cache_link.value[i].title) {
            cache_link.value[i].title = cache_link.value[i].name
          }
        }
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



  async function fetchUsers (item: any) {
    console.log(item)
    if (item.level == 0) {
      let data:any = await invoke("get_db_num", { redisUri: item.nname })
      console.log(data)
      let temp:any[] = []
      if (!data.is_success) {
          await message("链接redis失败！", { title: '提示', kind: 'error' })
          //upTreeVer();
          return [];
      }
      for (const [key, value] of Object.entries(data.data)) {
          temp.push({
              "leaf":false,
              "num":value,
              "name":key + "[ " + value + " ]",
              "db": key,
              "nname": key,
              "redis_uri":item.nname,
              "children":[],
              "level": item.level + 1,
              "json": key,
          });
      }
      console.log("------temp--------")
      console.log(temp)
      item.children = temp;
      return temp;
    } else if (item.level == 1) {
      let data:any = await invoke("get_keys", { key: "*", redisUri: item.redis_uri, db: item.db })
      console.log("--------------")
      console.log(data)
      if (data.is_success) {
          if (data.data && data.data.length > 0) {
            for (let i in data.data) {
              data.data[i].level = item.level + 1;
              data.data[i].json = {nname: data.data[i].nname, redis_uri: data.data[i].redis_uri, db: data.data[i].db}
            }
            item.children = data.data;
            return data.data
          }
          return [];
      } else {
          await message("链接redis失败！", { title: '提示', kind: 'error' })
          //upTreeVer();
          return [];
      }
    } else if(item.level > 1) {
      let data:any = await invoke("get_keys", { key: item.nname + "*", redisUri: item.redis_uri, db: item.db })
      console.log("--------------")
      console.log(data)
      if (data.is_success) {
          if (data.data && data.data.length > 0) {
            for (let i in data.data) {
              data.data[i].level = item.level + 1;
              data.data[i].json = {nname: data.data[i].nname, redis_uri: data.data[i].redis_uri, db: data.data[i].db}
            }
            item.children = data.data;
            return data.data
          }
          return [];
      } else {
          await message("链接redis失败！", { title: '提示', kind: 'error' })
          //upTreeVer();
          return [];
      }
    }
  }

  function addListFun() {
    overlay.value = true;
    formType.value = "addr";
  }

  function upLinkFun(data: any) {
    fromDataRef.id = data.id;
    fromDataRef.linkName = data.name;
    fromDataRef.host = data.host;
    fromDataRef.port = data.port;
    fromDataRef.password = data.password;
    fromDataRef.username = data.username;
    fromDataRef.linkType = data.linkType;
    fromDataRef.listenRedis = data.listenRedis;
    if (data.level == undefined) {
      fromDataRef.key = ("db库：" + data.db + "   " + data.nname);
    } else {
      fromDataRef.key = data.nname;
    }
    fromDataRef.level = data.level;
    fromDataRef.db = data.db;
    overlay.value = true;
    formType.value = "upAddr";
  }


  async function delLinkFun(data: any) {
    // Do you have permission to send a notification?
    let permissionGranted = await isPermissionGranted();
    // If not we need to request it
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }
    // Once permission has been granted we can send the notification
    if (permissionGranted) {
        sendNotification({ title: 'Tauri', body: 'Tauri is awesome!' });
    }
    const answer = await ask("是否删除 【" + data.name + "】", {
        title:"提示",
        kind: "warning"
    })
    console.log(answer)
    if (answer) {
        cache_link.value = cache_link.value.filter(item => item.id !== data.id)
        saveCacheFile(JSON.stringify(cache_link.value));
    }
  }


  watch(overlay, (n, _) => {
    if (!n) {
      formType.value = "";
    } 
  })


//添加redis方法
//"redis://192.168.5.126:6379/1";
function addLinkSubmit() {
   cache_link.value.push(
        {
            id: new Date().getTime(),
            name: fromData.linkName,
            nname: "redis://" + ((fromData.password) ? ( ":" + fromData.password + "@") : "") + fromData.host + ":" + fromData.port,
            leaf: false,
            listenRedis: fromData.listenRedis,
            password: fromData.password,
            host: fromData.host,
            port: fromData.port,
            level: 0,
        }
    )
    console.log(cache_link.value)
    saveCacheFile(JSON.stringify(cache_link.value));
    overlay.value = false;
    clearFormData
}

function addCancel() {
    overlay.value = false;
    clearFormData();
}


async function upLinkSubmit() {

  for (const i in cache_link.value) {
        if (cache_link.value[i].id == fromDataRef.id) {
            cache_link.value[i].name = fromDataRef.linkName;
            cache_link.value[i].nname = "redis://" + ((fromDataRef.password) ? ( ":" + fromDataRef.password + "@") : "") + fromDataRef.host + ":" + fromDataRef.port;
            cache_link.value[i].leaf = false;
            cache_link.value[i].listenRedis = fromDataRef.listenRedis;
            cache_link.value[i].password = fromDataRef.password;
            cache_link.value[i].host = fromDataRef.host;
            cache_link.value[i].port = fromDataRef.port;
            cache_link.value[i].level = 0;
            if (fromDataRef.listenRedis) {

            }
            let data:any = await invoke("reset_client", { redisUri: cache_link.value[i].nname });
            console.log(data);
            if (!data.is_success) {
              alert(data.msg)
              return
            }
        }
    }
    saveCacheFile(JSON.stringify(cache_link.value));
    overlay.value = false;
    clearFormDataRef
}

function upCancel() {
    overlay.value = false;
    clearFormDataRef();
}


const handleNodeClick = async (nodes: unknown) => {
  const activatedNodes = Array.isArray(nodes) ? nodes : [nodes];
  const clickedNode = activatedNodes[0];  // Get the first activated node
  // Check if the clicked node is a leaf (no children)
  if (!clickedNode.children || clickedNode.children.length === 0) {
    let result:any = await invoke("get_value", { key: clickedNode.nname, redisUri: clickedNode.redis_uri, db: clickedNode.db });
    console.log(result)
    content.value = result.data;
  }
};

//添加链接按钮
const isHovered = ref(false);
const handleMouseOver = () => {
  isHovered.value = true;
};
const handleMouseLeave = () => {
  isHovered.value = false;
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

  .v-navigation-drawer__contente::-webkit-scrollbar {
    width: 1px !important;
    height: 1px !important; /* 水平滚动条 */
  }


  .resizer {
    display: flex;
    cursor: ew-resize;
    height: 100vh;
  }

  .image-uploader {
  position: relative;
  height: 20px;
  border: 2px dashed #ccc;
  border-radius: 8px;
  display: flex;
  justify-content: center;
  align-items: center;
  transition: border-color 0.3s;
}

.image-uploader.hovered {
  border-color: #265abb; /* 当鼠标移入时边框颜色变为红色 */
}

  </style>
  