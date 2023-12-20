<template>
  <div v-for="course in courseList" :key="course.Code">
    {{ JSON.stringify(course) }}
  </div>
</template>

<script setup>
import setupReq from '../hooks/setupReq.js'

const courseList = setupReq('programs/3778/Computer Science', 'GET')

</script>
