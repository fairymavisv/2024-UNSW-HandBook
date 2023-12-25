import { ref, onMounted } from 'vue';
import useGlobalProp from './useGlobalProp.js'

export default function (callback, ...args) {
    const $fetchReq = useGlobalProp('$fetchReq')

    const data = ref()

    onMounted(async () => {
        const courseList = await $fetchReq(...args)
        data.value = callback(courseList)
    })

    return data
}