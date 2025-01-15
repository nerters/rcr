<script setup lang="ts">
import { onBeforeMount, reactive, Ref, ref } from "vue";
import { invoke } from '@tauri-apps/api/core';
import type Node from 'element-plus/es/components/tree/src/model/node'
import { writeTextFile, readTextFile, create, exists, BaseDirectory } from '@tauri-apps/plugin-fs';
import { message, ask } from "@tauri-apps/plugin-dialog";
import { ElTree } from "element-plus";

import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/plugin-notification";


const addLink = ref(false);
const upLink = ref(false)
const content = ref("");
const cacheFileName = "redis_links.json";
const cache_link:Ref<Array<any>> = ref([]);
const treeVer = ref(0);

onBeforeMount(async () => {
    let data = await getCacheFile()
    if (data) {
        // 转成json字符串并格式化
        cache_link.value = JSON.parse(data);
    } else {
        cache_link.value = [];
    }
    upTreeVer();
})

function upTreeVer() {
    treeVer.value++;
    console.log("-----------" + treeVer.value)
    console.log(cache_link.value)
    
}


interface Tree {
    label: string
    children?: Tree[]
    redis_uri: string
    db: string
}

const handleNodeClick = async (data: Tree) => {
    if (data.leaf) {
        console.log(data.nname)
        console.log(data.redis_uri)
        let result:any = await invoke("get_value", { key: data.nname, redisUri: data.redis_uri, db: data.db });
        content.value = result.data;
    }
}


const sizeForm = reactive({
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


function clearFormData() {
    sizeForm.id = "";
    sizeForm.linkName = "";
    sizeForm.host = "";
    sizeForm.port = "";
    sizeForm.password = "";
    sizeForm.username = "";
    sizeForm.linkType = "";
    sizeForm.listenRedis = false;
    sizeForm.key = "";
    sizeForm.level = 0;
}



interface Tree {
    nname: string
    name: string
    leaf?: boolean
}

const props = {
    label: 'name',
    children: 'zones',
    isLeaf: 'leaf',
}

const loadNode = async (
    node: Node,
    resolve: (data: Tree[]) => void,
    reject: () => void
) => {
    console.log(node.data);
    if (node.level === 0) {
        //let data:Array<any> = await invoke("get_keys", { key: "*" })
        let data = await getCacheFile()
        if (data) {
            // 转成json字符串并格式化
            cache_link.value = JSON.parse(data);
        } else {

            cache_link.value = [];
        }
        return resolve(cache_link.value)
    }
    if (node.level == 1) {
        console.log(node.data.nname)
        let data:any = await invoke("get_db_num", { redisUri: node.data.nname })
        console.log(data)
        let temp:any[] = []
        if (!data.is_success) {
            await message("链接redis失败！", { title: '提示', kind: 'error' })
            //upTreeVer();
            return reject()
        }
        for (const [key, value] of Object.entries(data.data)) {
            temp.push({
                "leaf":false,
                "num":value,
                "name":key + "[ " + value + " ]",
                "db": key,
                "nname":"",
                "redis_uri":node.data.nname
            });
        }
        return resolve(temp)
    } else if (node.level == 2) {
        let data:any = await invoke("get_keys", { key: "*", redisUri: node.data.redis_uri, db: node.data.db })
        console.log(data)
        if (data.is_success) {
            if (data.data && data.data.length > 0) {
                return resolve(data.data)
            }
            return reject()
        } else {
            await message("链接redis失败！", { title: '提示', kind: 'error' })
            //upTreeVer();
            return reject()
        }
    } else if (node.level > 2) {
        
        let data:any = await invoke("get_keys", { key: node.data.nname + "*", redisUri: node.data.redis_uri, db: node.data.db })
        if (data.is_success) {
            if (data.data && data.data.length > 0) {
                return resolve(data.data)
            }
            return reject()
        } else {
            await message("链接redis失败！", { title: '提示', kind: 'error' })
            //upTreeVer();
            return reject()
        }
    }
}
const asideWidth = ref(200); // 左侧区域初始宽度

// 鼠标按下时触发的事件
const onMouseDown = (e: MouseEvent) => {
    const startX = e.clientX;
    const startWidth = asideWidth.value;

    // 鼠标移动事件处理
    const onMouseMove = (moveEvent: MouseEvent) => {
    const offsetX = moveEvent.clientX - startX;
    // 限制宽度在 100px 到 (窗口宽度 - 100) 之间
    asideWidth.value = Math.max(100, Math.min(window.innerWidth - 100, startWidth + offsetX));

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
//添加链接按钮
const isHovered = ref(false);
const handleMouseOver = () => {
  isHovered.value = true;
};
const handleMouseLeave = () => {
  isHovered.value = false;
};
//添加方法
//"redis://192.168.5.126:6379/1";
function addLinkSubmit() {
   cache_link.value.push(
        {
            id: new Date().getTime(),
            name: sizeForm.linkName,
            nname: "redis://" + ((sizeForm.password) ? ( ":" + sizeForm.password + "@") : "") + sizeForm.host + ":" + sizeForm.port,
            leaf: false,
            listenRedis: sizeForm.listenRedis,
            password: sizeForm.password,
            host: sizeForm.host,
            port: sizeForm.port,
            level: 0,
        }
    )
    console.log(cache_link.value)
    saveCacheFile(JSON.stringify(cache_link.value));
    addLink.value = false;
    upTreeVer();
    clearFormData
}

function addCancel() {
    addLink.value = false;
    clearFormData();
}

//添加方法
//"redis://192.168.5.126:6379/1";
function upLinkSubmit() {
    for (const i in cache_link.value) {
        if (cache_link.value[i].id == sizeForm.id) {
            cache_link.value[i].name = sizeForm.linkName;
            cache_link.value[i].nname = "redis://" + ((sizeForm.password) ? ( ":" + sizeForm.password + "@") : "") + sizeForm.host + ":" + sizeForm.port;
            cache_link.value[i].leaf = false;
            cache_link.value[i].listenRedis = sizeForm.listenRedis;
            cache_link.value[i].password = sizeForm.password;
            cache_link.value[i].host = sizeForm.host;
            cache_link.value[i].port = sizeForm.port;
            cache_link.value[i].level = 0;
            if (sizeForm.listenRedis) {

            }
        }
    }
    saveCacheFile(JSON.stringify(cache_link.value));
    upLink.value = false;
    upTreeVer();
    clearFormData
}

function upCancel() {
    upLink.value = false;
    setTimeout(() => {
        clearFormData();
    }, 1000)
    
}

function updateLink(node: any, data: any) {
    console.log(data)
    sizeForm.id = data.id;
    sizeForm.linkName = data.name;
    sizeForm.host = data.host;
    sizeForm.port = data.port;
    sizeForm.password = data.password;
    sizeForm.username = data.username;
    sizeForm.linkType = data.linkType;
    sizeForm.listenRedis = data.listenRedis;
    if (data.level == undefined) {
        sizeForm.key = ("db库：" + data.db + "   " + data.nname);
    } else {
        sizeForm.key = data.nname;
    }
    sizeForm.level = data.level;
    sizeForm.db = data.db;
    upLink.value = true;
}

//删除
async function removeLink(node: any, data: any) {
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
        upTreeVer();
    }
}




</script>

<template>


<div class="common-layout" style="flex: 1">
    <el-container>
        <el-aside :style="{ width: asideWidth + 'px', backgroundColor: '#e9e9e9' }">
            <div class="image-uploader" :class="{ 'hovered': isHovered }" @mouseover="handleMouseOver" @mouseleave="handleMouseLeave" @click="addLink = true">
                <div class="add-image">
                <span>+</span>
                </div>

            </div>
            <el-tree :style="{maxWidth: asideWidth + 'px'}" :props="props" :load="loadNode" lazy 
            @node-click="handleNodeClick" :expand-on-click-node="false" :key="treeVer">
                <template #default="{ node, data }">
                    <span class="custom-tree-node">
                    <span>{{ node.label }}</span>
                    <span>
                        <!-- <a @click="append(data)"> Append </a> -->
                        <a v-if="node.level == 1" 
                        style="margin-left: 8px; display: inline-flex; align-items: center; justify-content: center; width: 15px; height: 10px;" 
                        @click="removeLink(node, data)">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" 
                                class="size-6" style="width: 100%; height: 100%; vertical-align: middle;">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                            </svg>
                        </a>
                    </span>
                    <span>
                        <a style="margin-left: 5px; display: inline-flex; width: 15px; height: 10px;" @click="updateLink(node, data)">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75" />
                            </svg>

                        </a>
                    </span>
                    </span>
                </template>
            </el-tree>
        </el-aside>

        <!-- 拖动条 -->
        <div class="resize-handle" @mousedown="onMouseDown"></div>
  
        <!-- Main 部分（右侧内容区） -->
        <el-main style="background-color: #fff; padding: 20px;">
          {{ content }}
        </el-main>

    </el-container>
</div>






<el-dialog v-model="addLink" title="添加卡片" width="380">

    <el-form ref="form" style="max-width: 600px" :model="sizeForm" label-width="auto" label-position="left" size="small" >
        <el-form-item label="链接名称">
            <el-input v-model="sizeForm.linkName" />
        </el-form-item>
        <el-form-item label="ip地址">
            <el-input v-model="sizeForm.host" />
        </el-form-item>
        <el-form-item label="端口">
            <el-input v-model="sizeForm.port" />
        </el-form-item>
        <el-form-item label="密码">
            <el-input v-model="sizeForm.password" />
        </el-form-item>
        <el-form-item label="监听">
            <el-switch v-model="sizeForm.listenRedis" style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949"/>
        </el-form-item>
        <el-form-item>
            <el-button type="primary" @click="addLinkSubmit">创建</el-button>
            <el-button @click="addCancel">取消</el-button>
        </el-form-item>
    </el-form>

</el-dialog>


<el-dialog v-model="upLink" title="修改卡片" width="380">

<el-form ref="form" style="max-width: 600px" :model="sizeForm" label-width="auto" label-position="left" size="small" >
    <el-form-item v-if="sizeForm.level == 0" label="链接名称">
        <el-input v-model="sizeForm.linkName" />
    </el-form-item>
    <el-form-item v-if="sizeForm.level == 0" label="ip地址">
        <el-input v-model="sizeForm.host" />
    </el-form-item>
    <el-form-item v-if="sizeForm.level == 0" label="端口">
        <el-input v-model="sizeForm.port" />
    </el-form-item>
    <el-form-item v-if="sizeForm.level == 0" label="密码">
        <el-input v-model="sizeForm.password" />
    </el-form-item>
    <el-form-item v-if="sizeForm.level != 0" label="监听key">
        <el-input disabled v-model="sizeForm.key" />
    </el-form-item>
    <el-form-item v-if="sizeForm.level != 0"  label="监听">
        <el-switch v-model="sizeForm.listenRedis" style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949"/>
    </el-form-item>
    <el-form-item>
        <el-button type="primary" @click="upLinkSubmit">更新</el-button>
        <el-button @click="upCancel">取消</el-button>
    </el-form-item>
</el-form>

</el-dialog>

</template>

<style scoped>
.el-tooltip__trigger {
margin-left: -15px;
}

.el-dropdown-menu__item {
height: 15px;
}

.common-layout {
    height: 100%;
}

.el-container {
height: 100%;
display: flex;
}

.el-aside{
background: #f4f4f4;
position: sticky;
top: 0;
overflow: auto;
}

.el-main {
background: #fff;
overflow-y: auto;
flex-grow: 1;
height: 100%;
}

.titlebar {
z-index: 1000;
height: 30px;
background: #ffffff;
user-select: none;
display: flex;
position: fixed;
top: 0;
left: 0;
right: 0;
justify-content: space-between;
align-items: flex-end;
}
.titleName {
display: flex;
justify-content: center;
align-items: center;

}
.titlebar-button {
display: inline-flex;
justify-content: center;
align-items: center;
width: 30px;
height: 30px;
user-select: none;
-webkit-user-select: none;
}
.titlebar-button:hover {
background: #a1a6a7;
}



.resize-handle {
  width: 10px;
  cursor: ew-resize;
  background-color: #ddd;
  height: 100%;
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

.image-container {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.image-container img {
  width: 100%;
  height: 100%;
  /* object-fit: cover; 等比例缩小图片 */
  object-fit: contain; /* 等比例缩小图片，保持原始宽高比 */
}


a svg path {
    transition: all 0.3s ease; /* 使颜色变化更加平滑 */
}

a:hover svg path {
    stroke: #f00; /* 鼠标移入时改变边框颜色 */
    fill: #f00;   /* 如果需要填充颜色，设置为红色 */
}
</style>