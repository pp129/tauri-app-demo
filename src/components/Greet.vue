<script setup>
import { ref,defineEmits } from "vue";
import { invoke,convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog"
// import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';


const name = ref("");
const imgList = ref([]);

const emit = defineEmits(['onAddItem']);

async function greet() {
  if(!name.value) return;
  invoke("add_item", { title: name.value }).then(()=>{
    emit('onAddItem')
    name.value = ""
  })
}

const uploadImage = async ()=>{
  let files = await open({
    multiple: true,
    filters: [{
      name: 'Image',
      extensions: ['png', 'jpeg','jpg']
    }]
  });
  if(files && files.length>0) {
    files.forEach(filePath => {
      const url = convertFileSrc(filePath)
      console.log(url)
      imgList.value.push({url})
    })
  }
}

const uploadExcel = async ()=>{
  const path = await open({
    filters: [{
      name: 'Excel',
      extensions: ['xlsx', 'xls']
    }]
  });
  // console.log(path);
  if(path) {
    const excelData = await invoke('read_excel', {path});
    console.log(excelData);
  }
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
    <button @click="uploadImage">upload Image</button>
    <button @click="uploadExcel">upload Excel</button>
  </form>
  <img v-for="(img,index) in imgList" :key="index" :src="img.url" alt="">
</template>

<style>
.row button {
  margin-right: 10px;
}
</style>
