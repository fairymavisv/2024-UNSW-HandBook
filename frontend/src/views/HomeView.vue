<template>
  <div class="navbar">
    <div class="navbar-brand">UNSW Handbook</div>
    <div class="search-bar">
      <input type="text" placeholder="Search...">
    </div>
    <div>
    <MenuDrawer />  
    </div>
  </div>
</template>

<script>
// @ is an alias to /src
import MenuDrawer from '../components/MenuDrawer.vue'

export default {
  name: 'HomeView',
  components: {
    MenuDrawer
  }
}
</script>

<style>
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #2c3e50; 
  padding: 10px 20px;
  color: white;
}

.navbar-brand {
  font-size: 24px;
  font-weight: bold;
}

.search-bar input {
  padding: 8px;
  border-radius: 4px;
  width: 100%;
  border: 1px solid #ddd;
}
</style>