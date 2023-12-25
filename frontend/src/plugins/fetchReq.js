export default function (app) {
    app.config.globalProperties.$fetchReq = async (url, method, body) => {
        const token = localStorage.getItem('token')

        console.group(url, method, body, token)

        const response = await fetch(`/api/` + url, {
            method,
            headers: {
                'Content-Type': 'application/json',
                Authorization: 'Bearer ' + token
            },
            body: body ? JSON.stringify(body) : null
        })
        const data = await response.json()

        console.table(data)
        console.groupEnd()

        return data;
    }
}
