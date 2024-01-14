<template>
    <div class="app">
        <aside class="sidebar">
            <Menu style="width: 3em; height: 3em; margin-top: 20px; margin-bottom: 30px;" />
            <Search style="width: 3em; height: 3em; margin-top: 20px;" />
            <User style="width: 3em; height: 3em; margin-top: 20px;" />
            <Star style="width: 3em; height: 3em; margin-top: 20px;" />
        </aside>
        <main class="main-content">
            <header class="header">
                <h2>{{ courseName }}</h2>
                <h5>{{ code }} | {{ UOC }} Units of Credit </h5>
            </header>
            <div class="course-info">
                <div class="comment">
                    <h3>Comments</h3>
                    <el-scrollbar height="40    0px">
                        <p v-for=" comment in comments" :key="comment.id" class="scrollbar-demo-item">{{ comment.text }}</p>
                    </el-scrollbar>
                </div>
                <div class="ratings">
                    <div class="rating-container">
                        <div class="rating">
                            <span>Difficulty:  </span>
                            <el-rate v-model="scores[0]" disabled show-score text-color="#ff9900"
                                score-template="{value} points" />
                        </div>
                        <div class="rating">
                            <span>Usefulness:   </span>
                            <el-rate v-model="scores[1]" disabled show-score text-color="#ff9900"
                                score-template="{value} points" />
                        </div>
                        <div class="rating">
                            <span>Workload:   </span>
                            <el-rate v-model="scores[2]" disabled show-score text-color="#ff9900"
                                score-template="{value} points" />
                        </div>
                    </div>
                    <div class="description">
                        <p>{{ description }}</p>
                    </div>
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

onMounted(async () => {
    const data = await $fetchReq("course/" + courseId, "GET");
    courseData.value = data;
    UOC.value = data.basicInfo.UOC; // 更新学分
    code.value = data.basicInfo.code; // 更新课程代码
    courseName.value = data.basicInfo.name; // 更新课程名称
    description.value = data.basicInfo.description; // 更新课程描述
    comments.value = data.comments; // 更新评论
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
.app {
    display: flex;
}

.sidebar {
    width: 80px;
    background-color: #e1e1e1;
    height: 100vh;
    padding-bottom: 20px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column-reverse;
    align-items: center;
}

.sidebar div {
    width: 50px;
    height: 50px;
    background-color: #555;
    margin-bottom: 10px;
}

.main-content {
    flex-grow: 1;
}

.header {
    background-color: #007bff;
    color: white;
    padding: 1em;
    text-align: center;
    height: 150px;
    font-size: 30px;
}

.comment {
    padding: 2em;
    font-size: 1.5em;
    width: 66%
}

.ratings {
    background-color: #9997e1;
    padding: 1em;
    font-size: 1.5em;
    width: 33%;
    height: calc(100vh - 210px - 2em);
}

.rating {
    margin: 1.5em 0;
}

.stars {
    color: white;
}

.star {
    color: grey;
}

.star.filled {
    color: gold;
}

.description {
    color: black;
    margin-top: 1em;
    font-size: 1em;
}

.course-info {
    display: flex;
    flex-direction: row;
}

.rating-container {
    margin-top: 5em;
    display: flex;
    flex-direction: column;
    height: 50%;
}

.scrollbar-demo-item {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100px;
    margin: 10px;
    text-align: center;
    border-radius: 4px;
    background: var(--el-color-primary-light-9);
    color: var(--el-color-primary);
}
</style>