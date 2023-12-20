<template>
  <div v-for="{ Code, UOC, 'Course Name': courseName } in courseList" :key="Code">
    {{ Code }} {{ UOC }} {{ courseName }}
  </div>
</template>

<script setup>
import setupReq from '../hooks/setupReq.js'

const courseList = setupReq('programs/3778/Computer Science', 'GET')

</script>
