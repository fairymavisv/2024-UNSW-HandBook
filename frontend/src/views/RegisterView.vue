<template>
<div class = 'register-container'>
    <div class = 'register-box'>
        <h2>Register</h2>
        <form>
            <div class = 'input-group'>
                <input type="text" placeholder="Zmail" required>
            </div>
            <div class = 'input-group'>
                <input type="password" placeholder="Password" required>
            </div>
            <div class = 'input-group'>
                <input type="password" placeholder="Confirm Password" required>
            </div>
            <button type="submit">Register</button>
        </form>
    </div>
</div>
</template>

<script>
export default{
    name:"RegisterView",
}
</script>

<style>
.register-container{
    position: fixed; 
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #77CFEE;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 200 200'%3E%3Cdefs%3E%3ClinearGradient id='a' gradientUnits='userSpaceOnUse' x1='100' y1='33' x2='100' y2='-3'%3E%3Cstop offset='0' stop-color='%23000' stop-opacity='0'/%3E%3Cstop offset='1' stop-color='%23000' stop-opacity='1'/%3E%3C/linearGradient%3E%3ClinearGradient id='b' gradientUnits='userSpaceOnUse' x1='100' y1='135' x2='100' y2='97'%3E%3Cstop offset='0' stop-color='%23000' stop-opacity='0'/%3E%3Cstop offset='1' stop-color='%23000' stop-opacity='1'/%3E%3C/linearGradient%3E%3C/defs%3E%3Cg fill='%2354aecd' fill-opacity='0.6'%3E%3Crect x='100' width='100' height='100'/%3E%3Crect y='100' width='100' height='100'/%3E%3C/g%3E%3Cg fill-opacity='0.5'%3E%3Cpolygon fill='url(%23a)' points='100 30 0 0 200 0'/%3E%3Cpolygon fill='url(%23b)' points='100 100 0 130 0 100 200 100 200 130'/%3E%3C/g%3E%3C/svg%3E");
}

.register-box {
    position: relative;
    width: 600px; 
    height: 300px; 
    padding-top: 40px; 
    background: #fff;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.register-box h2 {
    position: absolute;
    top: 30px;
    left: 50%;
    transform: translateX(-50%);
    font-size: 35px;
    color: #2c3e50;
    margin: 0;
}

.register-box form {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
}

.input-group {
    margin-bottom: 15px;
    width: 80%; 
}

.input-group input{
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    box-sizing: border-box;
}

button {
    width: 80%; 
    padding: 10px;
    margin-top: 10px;
    border: none;
    background: #007bff;
    color: white;
    border-radius: 4px;
    cursor: pointer;
}

button:hover {
    background: #0056b3;
}
</style>