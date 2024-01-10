export default function (app) {
    const fetchReq = async (url, method, body, refreshFlag=false) => {
        const key = refreshFlag ? 'refreshToken' : 'accessToken'
        const token = localStorage.getItem(key)

        console.group(url, method, body, refreshFlag, token)

        const response = await fetch(`/api/` + url, {
            method,
            headers: {
                'Content-Type': 'application/json',
                [key]: token
            },
            body: body ? JSON.stringify(body) : null
        })

        let data = await response.json()

        console.table(data)

        if (data.statusCode === 401) {

            console.log('refreshToken')
            data = await fetchReq('auth/refreshToken', 'POST', null, true)
            
            if (data.statusCode !== 200) {
                console.log('refresh token failed, redirect to login page')
                // this.$router.push('/login')
            } else {
                localStorage.setItem('accessToken', data.accessToken)
                localStorage.setItem('refreshToken', data.refreshToken)

                console.log('refresh token succeed, request retry')
                data = await fetchReq(url, method, body)
            }
        }

        console.groupEnd()

        return data;
    }

    app.config.globalProperties.$fetchReq = fetchReq;
}
