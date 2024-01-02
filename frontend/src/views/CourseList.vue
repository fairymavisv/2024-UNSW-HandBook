<template>
  <!-- <div id="filterContainer">
    <div>
      <span>Offer terms: </span>

      <input type="checkbox" id="term 1" :value="1" v-model="termfilter">
      <label for="term 1">Term 1</label>

      <input type="checkbox" id="term 2" :value="2" v-model="termfilter">
      <label for="term 2">Term 2</label>

      <input type="checkbox" id="term 3" :value="3" v-model="termfilter">
      <label for="term 3">Term 3</label>
    </div>

    <div>
      <span>
        UOC: 
        <input type="number" min="0" max="6" style="width: 50px" >
      </span>
    </div>

  </div> -->

  <div class="common-layout">
    <el-container>
      <el-header style="background-color: rgb(174, 174, 237)">Header</el-header>
      <el-container>
        <el-aside width="200px" style="background-color: rgb(174, 174, 237)"
          >Aside</el-aside
        >
        <el-main>
          <el-table :data="courseList" style="margin: 0 auto; width: 85%">
            <el-table-column prop="Code" label="Code"></el-table-column>
            <el-table-column
              prop="Course Name"
              :width="500"
              label="Name"
            ></el-table-column>
            <el-table-column
              prop="uoc"
              label="UOC"
              :filter-multiple="false"
              :filters="[
                { text: '1', value: 1 },
                { text: '2', value: 2 },
                { text: '3', value: 3 },
                { text: '4', value: 4 },
                { text: '5', value: 5 },
                { text: '6', value: 6 },
              ]"
              :filter-method="(uoc, course) => course.uoc === uoc"
            ></el-table-column>
            <el-table-column
              prop="term"
              label="Offer Terms"
              :filter-multiple="false"
              :filters="[
                { text: 'Term 1', value: 1 },
                { text: 'Term 2', value: 2 },
                { text: 'Term 3', value: 3 },
              ]"
              :filter-method="(term, course) => course.term.includes(term)"
            ></el-table-column>
          </el-table>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup>
import { ref } from "vue";
import useGlobalProp from "../hooks/useGlobalProp.js";

const $fetchReq = useGlobalProp("$fetchReq");

const courseList = ref([]);

//  onMounted(async () => {
// 	const data = await $fetchReq('programs/3778/Computer Science', 'GET')

// 	data.forEach(course => {
// 		const terms = Math.ceil(Math.random() * 3)
// 		course.term = []
// 		for (let i = 1; i <= terms; i++) {
// 			course.term.push(i)
// 		}

// 		course.uoc = Math.ceil(Math.random() * 6)
// 	})

// 	courseList.value = data
// })

$fetchReq("programs/3778/Computer Science", "GET").then((data) => {
  data.forEach((course) => {
    const terms = Math.ceil(Math.random() * 3);
    course.term = [];
    for (let i = 1; i <= terms; i++) {
      course.term.push(i);
    }

    course.uoc = Math.ceil(Math.random() * 6);
  });

  courseList.value = data;
});
</script>

<style scoped>
/* #filterContainer {
  display: flex;
  flex-direction: row;
  justify-content: center;
}

#filterContainer > div {
  border: 1px solid black;
  padding: 10px;
  border-radius: 30px;
}

#filterContainer > div > span {
  font-weight: bold;
} */
</style>
