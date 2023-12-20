<template>
  <div>
    {{ courseList }}
  </div>
</template>

<script setup>
import setupReq from '../hooks/setupReq.js'

const courseList = setupReq('programs/3778/Computer Science', 'GET')

</script>
