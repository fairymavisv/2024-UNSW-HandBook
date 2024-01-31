import { getCurrentInstance, onMounted } from "vue"

export default function(key) {
    onMounted(() => { console.log(`get ${key} plugin`) })
    
    const {appContext: {config: {globalProperties: {[key] : value}}}} = getCurrentInstance();
    // return instance.appContext.config.globalProperties[key];
    return value
}