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
          <v-expansion-panel-title v-slot="{ expanded }" @click="fetchUsers(link)" class="position-sticky top-0 pa-3 mt-2" style="z-index: 20; background-color:silver">
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
                <v-icon v-if="item.children">
                  {{ isOpen ? 'mdi-folder-open' : 'mdi-folder' }}
                </v-icon>
                <v-icon v-else>
                  mdi-file-document-outline
                </v-icon>
              </template>
            </v-treeview>

          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>
      </v-navigation-drawer>
  
  
      <v-divider :thickness="8" vertical class="resizer" :style="{ 'padding-left': drawerWidth + 'px' }" @mousedown="onMouseDown"></v-divider>
  
      <!-- Main Content -->
      <v-main class="d-flex justify-center" style="padding-left: 0px;">

        <div v-if="details" style="width: 100%; height: 100%;">
          <v-row no-gutters justify="space-between">
            <v-col cols="12" sm="1" class="d-flex align-center">
              key:
            </v-col>

            <v-col cols="12" sm="7">
              <!-- <text>{{ key }}</text> -->
              <v-text-field 
                :append-inner-icon="key == sourceKey ? '' : 'mdi-check'"
                variant="underlined" 
                :clear-icon="key == sourceKey ? '' : 'mdi-close-circle'"
                clearable
                v-model:model-value="key"
                @click:clear="clearKey"
                @click:append-inner="upKey"
              ></v-text-field>
            </v-col>

            

            <v-col cols="12" sm="1" class="d-flex align-center">
              ttl:
            </v-col>

            <v-col cols="12" sm="3">
              <v-text-field 
                :append-inner-icon="ttl == sourceTtl ? '' :'mdi-check'"
                variant="underlined" 
                :clear-icon="ttl == sourceTtl ? '' : 'mdi-close-circle'"
                clearable
                v-model:model-value="ttl"
                @click:clear="clearTTL"
                @click:append-inner="upTTL"
              ></v-text-field>
            </v-col>

          </v-row>

          <text>{{ content }}</text>

        </div>

       
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
import setPromiseInterval from 'set-promise-interval';

  class Tree {
    public id!: number;
		public name!: String;
		public nname!: String;
		public leaf!: boolean;
		public listenRedis!: boolean;
		public password!: String;
		public host!: String;
		public port!: String;
		public level!: number;
		public children!: Array<Tree>;
		public title!: String;
  }

  const drawerWidth = ref(230);
  
  const cacheFileName = "redis_links.json";
  const cache_link:Ref<Array<Tree>> = ref([]);
  const initiallyOpen = ref(['public']);
  const redisUri = ref("");
  const db = ref("");

  const active = ref([]);
  //选中数节点值
  const itemInfo = ref();
  //key
  const key = ref("");
  const sourceKey = ref("");
  //值
  const content = ref("");
  const sourceContent = ref("");
  //剩余存活时间
  const ttl = ref(-1);
  const sourceTtl = ref(-1);
  //值展示时间 单位秒
  const showTime = ref(0);
  //详情页
  const details = ref(false);
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
          id: 0,
          name: '.git',
          nname: '.git',
          leaf: false,
          listenRedis: false,
          password: "",
          host: "",
          port: "",
          level: 0,
          children: []
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

  function clearKey() {
    key.value = sourceKey.value;
  }

  async function upKey() {
    console.log(itemInfo)
    let asr = await ask('是否要把 ' + sourceKey.value + " 替换为 " + key.value + " ?", {
      title: '替换key',
      kind: 'warning',
    });
    if (asr) {
      let res:any = await invoke("reset_key_name", { redisUri: redisUri.value, db: db.value, key: key.value, sourceKey: sourceKey.value });
      if (res.is_success) {
        changeTree(itemInfo.value.redis_uri, db.value, sourceKey.value, key.value);

        sourceKey.value = key.value;
        itemInfo.value.nname = key.value;
        itemInfo.value.name = key.value;
        itemInfo.value.json = {nname: itemInfo.value.nname, redis_uri: itemInfo.value.redis_uri, db: itemInfo.value.db};
        console.log(itemInfo.value);
      }
    }
  }


  function clearTTL() {
    ttl.value = sourceTtl.value;
  }

  async function upTTL() {
    let asr = await ask('是否要把存活时间修改为 ' + ttl.value + " ?", {
      title: '修改存活时间',
      kind: 'warning',
    });
    if (asr) {
      console.log("修改!");
      let res:any = await invoke("reset_ttl_by_key", { redisUri: redisUri.value, db: db.value, key: key.value, ttl: ttl.value ? parseInt(ttl.value.toString()) : -1 });
      console.log(res)
      if (res.is_success) {
        sourceTtl.value = ttl.value;
      }
    }
  }

  //树下拉
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
      redisUri.value = item.redis_uri;
      db.value = item.db;
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
      redisUri.value = item.redis_uri;
      db.value = item.db;
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
          nname: "redis://" + ((fromData.password) ? (":" + fromData.password + "@") : "") + fromData.host + ":" + fromData.port,
          leaf: false,
          listenRedis: fromData.listenRedis,
          password: fromData.password,
          host: fromData.host,
          port: fromData.port,
          level: 0,
          children: [],
          title: fromData.linkName,
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
      if (cache_link.value[i].id.toString() == fromDataRef.id) {
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

function getItemByNname(list: Array<Tree>, nname: String) {
  if (!list) {
    console.log("查找的item的子集为空！")
    return null;
  }
  console.log("待查询list")
  console.log(list)
  console.log("待查询nname")
  console.log(nname)
  for (let i in list) {
    if (list[i].nname == nname) {
      return list[i];
    }
  }
  return null;
}

async function changeTree(redisUrl: String, db: String, sourceKey: String, key: String) {
  let split = sourceKey.split(":");
  let item = getItemByNname(cache_link.value, redisUrl);
  if (!item) {
    console.log(redisUrl)
    alert("修改树失败！");
    return;
  }
  item = getItemByNname(item.children, db);
  if (!item) {
    console.log("获取db数据失败！")
    alert("修改树失败！");
    return;
  }
  let data:any = await invoke("get_db_num", { redisUri: redisUrl })
  if (data.is_success) {
    item.name = db + "[ " + data.data[parseInt(db.toString())] + " ]";
  }


  let cache_key = "";
  let last_parent_key = "";
  let last_parent = item;
  for (let i in split) {
    if (!item) {
      console.log(redisUrl)
      alert("修改树失败！");
      return;
    }
    if (parseInt(i) == (split.length - 2)) {
      last_parent = item;
      last_parent_key = cache_key + "*";
    }
    cache_key += parseInt(i) == (split.length - 1) ? split[i] : (split[i] + ":");
    item = getItemByNname(item.children, cache_key);
  }

  console.log(item);
  if (item?.name == sourceKey) {
    console.log("找到了！")
    if (last_parent) {
      last_parent.children = last_parent.children.filter(item => item.nname !== key);
    }
    
    item.name = key;
    item.nname = key;

    
    // if (last_parent) {
    //   last_parent.children.length = 0;
    //   let data:any = await invoke("get_keys", { key: last_parent_key, redisUri: redisUrl, db: db })
    //   if (data.is_success) {
    //       if (data.data && data.data.length > 0) {
    //         for (let i in data.data) {
    //           data.data[i].level = last_parent.level + 1;
    //           data.data[i].json = {nname: data.data[i].nname, redis_uri: data.data[i].redis_uri, db: data.data[i].db}
    //         }
    //         last_parent.children = data.data;
    //       }
    //       last_parent.children = [];
    //   }
    // } else {
    //   console.log("寻找到的item的父级失败！")
    //   alert("修改树失败！");
    // }
  } else {
    console.log("寻找到的item的name与sourceKey对应不上")
    alert("修改树失败！");
  }
}


//点击叶子节点
const handleNodeClick = async (nodes: unknown) => {
  const activatedNodes = Array.isArray(nodes) ? nodes : [nodes];
  const clickedNode = activatedNodes[0];  // Get the first activated node
  // Check if the clicked node is a leaf (no children)

  console.log(":::::::::::::::::::")
  console.log(active)
  itemInfo.value = clickedNode;
  if (clickedNode == undefined) {
    details.value = false;
    content.value = "";
    sourceContent.value = "";
    ttl.value = -1;
    sourceTtl.value = -1;
    key.value = "";
    sourceKey.value = "";
    redisUri.value = "";
    db.value = "";
  } else if (!clickedNode.children || clickedNode.children.length === 0) {
    
    details.value = true;
    let result:any = await invoke("get_value", { key: clickedNode.nname, redisUri: clickedNode.redis_uri, db: clickedNode.db });
    console.log(result)
    if (!result.is_success) {
      if (result.code == -12) {
        await message(result.msg, { title: 'key不存在', kind: 'error' });
        changeTree(clickedNode.redis_uri, db.value, clickedNode.nname, clickedNode.nname);
      }
    } else {
      content.value = result.data[0];
      sourceContent.value = result.data[0];
      ttl.value = result.data[1];
      sourceTtl.value = result.data[1];
      key.value = clickedNode.nname;
      sourceKey.value = clickedNode.nname;
      redisUri.value = clickedNode.redis_uri;
      db.value = clickedNode.db;
    }

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


  // 每秒更新一次时间戳
  setPromiseInterval( async () => {
    showTime.value += 1;
    if (sourceTtl.value.toString() != "-1" && sourceTtl.value.toString() != "0") {
      if (sourceTtl.value.toString() == ttl.value.toString()) {
        ttl.value = parseInt(ttl.value.toString()) - 1;
      }
      sourceTtl.value = parseInt(sourceTtl.value.toString()) - 1;
    }
  }, 1000);

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
  