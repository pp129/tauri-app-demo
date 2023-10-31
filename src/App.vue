<script setup>
import {ref} from 'vue';
import { invoke } from "@tauri-apps/api/tauri";
import Greet from "./components/Greet.vue";

let todoList = ref([]);

const loadList = function () {
  invoke("list").then(result => {
    console.log("list: " , result)
    todoList.value = [...result]
  })
  console.log(todoList)
}

const delItem = async function ({id}) {
  invoke("del", { id }).then(()=>{
    loadList()
  })
}

loadList();

</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <p>
      Recommended IDE setup:
      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
      +
      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
      +
      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
        >Tauri</a
      >
      +
      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
        >rust-analyzer</a
      >
    </p>

    <Greet @onAddItem="loadList" />

    <p v-for="item in todoList" :key="item.id" @dblclick="delItem(item)">{{ item.title }}</p>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
