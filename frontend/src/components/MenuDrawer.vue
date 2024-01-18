<template>
    <div>
      <v-btn @click="toggleDrawer(true)">Menu</v-btn>
      <v-navigation-drawer
        v-model="drawer"
        :temporary="true"
        app
      >
        <v-list>
          <v-list-item
            v-for="(item, index) in menuItems"
            :key="index"
            @click="toggleDrawer(false)"
          >
            <v-list-item-icon>
              <v-icon>{{ item.icon }}</v-icon>
            </v-list-item-icon>
            <v-list-item-content>
              <v-list-item-title>{{ item.text }}</v-list-item-title>
            </v-list-item-content>
          </v-list-item>
        </v-list>
      </v-navigation-drawer>
    </div>
  </template>
  
  <script>
  import { ref } from 'vue';
  
  export default {
    name: 'MenuDrawer',
    setup() {
      const drawer = ref(false);
  
      const menuItems = [
        { text: 'Inbox', icon: 'mdi-inbox' },
        { text: 'Starred', icon: 'mdi-star' },
        { text: 'Send email', icon: 'mdi-send' },
        { text: 'Drafts', icon: 'mdi-file-outline' },
      ];
  
      const toggleDrawer = (open) => {
        drawer.value = open;
      };
  
      return { drawer, menuItems, toggleDrawer };
    },
  };
  </script>
  