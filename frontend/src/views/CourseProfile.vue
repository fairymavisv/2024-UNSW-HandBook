<template>
    <div class="app">
        <el-divider />
        <main class="main-content">
            <div class="first-row">
                <div class="rounded-border-1">
                    <div class="rounded-border-course">
                        <h1>{{ courseName }}</h1>
                    </div>

                    <div class="rounded-border-small">
                        <h3>{{ code }}</h3>
                    </div>

                    <div class="rounded-border-small">
                        <h3>{{ UOC }} UOC</h3>
                    </div>

                </div>
                <div class="rounded-border-2">
                    <div class="rating">
                        <span>Difficulty: </span>
                        <el-rate v-model="scores[0]" disabled show-score text-color="#ff9900"
                            score-template="{value} points" />
                    </div>
                    <div class="rating">
                        <span>Usefulness: </span>
                        <el-rate v-model="scores[1]" disabled show-score text-color="#ff9900"
                            score-template="{value} points" />
                    </div>
                    <div class="rating">
                        <span>Workload: </span>
                        <el-rate v-model="scores[2]" disabled show-score text-color="#ff9900"
                            score-template="{value} points" />
                    </div>
                </div>
            </div>
            <div class="second-row">
                <div class="rounded-border-3">
                    <div class="rounded-border-small hover" @click="current = 'summery'">
                        <h3>summery</h3>
                    </div>
                    <div class="rounded-border-small hover" @click="current = 'comments'">
                        <h3>comments</h3>
                    </div>
                </div>

                <div class="rounded-border-4">
                    <div v-if="current === 'summery'" class="rounded-border-small">
                        <h3>description:</h3>
                        <p>{{ description }}</p>
                    </div>
                    <div v-if="current === 'summery'" class="rounded-border-small">
                        <h3>offerterms:</h3>
                        <p v-for="offerterm in offerterms" :key="offerterm">{{ offerterm }}&nbsp;</p>
                    </div>

                    <div v-if="current === 'comments'" class="rounded-border-small">
                        <h3>comments</h3>
                    </div>
                    <el-scrollbar v-if="current === 'comments'" height="200px">
                        <div v-for="item in comments" :key="item" class="scrollbar-demo-item"><h3>{{ item.text }}</h3></div>
                    </el-scrollbar>
                </div>
            </div>
        </main>
    </div>
</template>
  
<script setup>
import { ref, onMounted } from 'vue';
import useGlobalProp from "../hooks/useGlobalProp.js";
import { useRoute } from 'vue-router';

const route = useRoute();
const courseId = route.params.courseId;
const scores = ref([2, 3, 4]);
const courseData = ref(null);
const courseName = ref('');
const code = ref('');
const UOC = ref(6);
const description = ref('This is a course description');
const $fetchReq = useGlobalProp("$fetchReq");
const comments = ref('This is a comment');
const Difficulty = ref(0);
const Usefulness = ref(0);
const Workload = ref(0);
const offerterms = ref('This is a offerterms');
const current = ref('summery');

onMounted(async () => {
    const data = await $fetchReq("course/" + courseId, "GET");
    courseData.value = data;
    UOC.value = data.basicInfo.UOC; // 更新学分
    code.value = data.basicInfo.code; // 更新课程代码
    courseName.value = data.basicInfo.name; // 更新课程名称
    description.value = data.basicInfo.description; // 更新课程描述
    comments.value = data.comments; // 更新评论
    offerterms.value = data.basicInfo.offerterms; // 更新offerterms
    for (let i = 0; i < data.comments.length; i++) {
        Difficulty.value += data.comments[i].difficulty;
        Usefulness.value += data.comments[i].usefulness;
        Workload.value += data.comments[i].workload;
    }
    Difficulty.value = Difficulty.value / data.comments.length;
    Usefulness.value = Usefulness.value / data.comments.length;
    Workload.value = Workload.value / data.comments.length;

    scores.value = [Difficulty.value, Usefulness.value, Workload.value];
    console.log(courseData.value);
});

</script>
  
<style scoped>
.first-row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}

.second-row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}

.rounded-border-1 {
    border: 2px solid #000000;
    border-radius: 10px;
    padding: 10px;
    width: 70%;
    height: 200px;
    background-color: #007bff;
    display: flex;
    flex-direction: row;
}

.rounded-border-2 {
    border: 2px solid #000000;
    border-radius: 10px;
    padding: 10px;
    width: 25%;
    height: 200px;
    background-color: #007bff;
}

.rounded-border-course {
    border: 2px solid #000000;
    border-radius: 30px;
    width: auto;
    height: 50%;
    background-color: white;
    align-items: center;
    justify-content: center;
    display: flex;
    padding: 0 5%;
    margin-top: 4%;
}

.rounded-border-small {
    border: 2px solid #000000;
    border-radius: 30px;
    width: auto;
    height: 20%;
    background-color: white;
    align-items: center;
    justify-content: center;
    display: flex;
    padding: 0 5%;
    margin-top: 4%;
    margin-left: 1%;
}

.rating {
    margin: 1.5em 0;
    border: 2px solid #000000;
    border-radius: 30px;
    background-color: white;
    width: auto;
    margin-left: 10%;
    margin-right: 10%;
}

.rounded-border-3 {
    border: 2px solid #000000;
    border-radius: 10px;
    padding: 10px;
    width: 20%;
    height: 300px;
    background-color: #007bff;
    margin-top: 5%;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
}

.rounded-border-4 {
    border: 2px solid #000000;
    border-radius: 10px;
    padding: 10px;
    width: 70%;
    height: 500px;
    background-color: #007bff;
    margin-top: 5%;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
}

.hover {
    cursor: pointer;
}

.scrollbar-demo-item {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 50px;
  margin: 10px;
  text-align: center;
  border-radius: 4px;
  background: white;
  color: black;
  border: 2px solid #000000;
    border-radius: 30px;
}
</style>