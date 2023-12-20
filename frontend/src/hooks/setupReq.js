import { ref, onBeforeMount} from 'vue';
import useGlobalProp from './useGlobalProp.js'

export default function (...args) {
    const $fetchReq = useGlobalProp('$fetchReq')
    
    const data = ref()

    onBeforeMount(async () => {
    const courseList = await $fetchReq(...args)
    data.value = courseList
    })

    return data
}