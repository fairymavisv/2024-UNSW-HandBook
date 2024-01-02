<template>
    <div id="background">
        <div id="container">
            <div id="form">
                <h1>{{ $route.path.slice(1) }}</h1>
                <p>Email</p>
                <input placeholder="zid@ad.unsw.edu.au" v-model="email" />
                <p>Password</p>
                <input type="password" placeholder="password" v-model="password" />
                <template v-if="isRegister">
                    <p>Confirm Password</p>
                    <input type="password" placeholder="confirm password" v-model="confirmPass" />
                    <p>Account Name</p>
                    <input placeholder="name" v-model="name" />
                </template>
                <button @click="enter">Enter</button>
            </div>
            <p id="tip">
                {{ isRegister ? 'Already' : 'Don\'t' }} have an account?
                <router-link :to="isRegister ? '/login' : '/register'">Go {{ isRegister ? 'login' :
                    'register' }}</router-link>
            </p>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            email: '',
            password: '',
            confirmPass: '',
            name: ''
        }
    },

    computed: {
        isRegister() {
            return this.$route.path === '/register';
        },

        validEmail() {
            const regex = /^z[0-9]{7}@ad\.unsw\.edu\.au$/;
            return regex.test(this.email);
        },

        validPassword() {
            const regex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,}$/;
            return regex.test(this.password);
        },

        validConfirmPass() {
            return this.password === this.confirmPass;
        },
    },

    methods: {
        async enter() {
            if (!this.validEmail) {
                this.$message({
                    message: 'Invalid email. Please use your offical UNSW email address',
                    type: 'warning'
                });
                return;
            }

            if (this.isRegister) {
                if (!this.validPassword) {
                    this.$message({
                        message: 'Invalid password. Password must be at least 8 characters long, contain at least 1 uppercase letter, 1 lowercase letter and 1 number',
                        type: 'warning'
                    });

                    return;
                }

                if (!this.validConfirmPass) {
                    this.$message({
                        message: 'Passwords do not match.',
                        type: 'warning'
                    });
                    return;
                }
            }

            const { email: username, password, name } = this;

            const data = await this.$fetchReq(
                'auth/' + (this.isRegister ? 'register' : 'login'),
                'POST',
                { username, password, name }
            );

            if (data.error) {
                this.$message({
                    message: data.error,
                    type: 'error'
                });
            } else {
                this.$message({
                    message: 'Success',
                    type: 'success'
                });

                localStorage.setItem('token', data.token);
                this.$router.push('/courseList');
            }
        }
    }
};
</script>

<style scoped>
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;

}

#background {
    padding: 0.1px;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 200 200'%3E%3Cdefs%3E%3ClinearGradient id='a' gradientUnits='userSpaceOnUse' x1='100' y1='33' x2='100' y2='-3'%3E%3Cstop offset='0' stop-color='%23000' stop-opacity='0'/%3E%3Cstop offset='1' stop-color='%23000' stop-opacity='1'/%3E%3C/linearGradient%3E%3ClinearGradient id='b' gradientUnits='userSpaceOnUse' x1='100' y1='135' x2='100' y2='97'%3E%3Cstop offset='0' stop-color='%23000' stop-opacity='0'/%3E%3Cstop offset='1' stop-color='%23000' stop-opacity='1'/%3E%3C/linearGradient%3E%3C/defs%3E%3Cg fill='%2354aecd' fill-opacity='0.6'%3E%3Crect x='100' width='100' height='100'/%3E%3Crect y='100' width='100' height='100'/%3E%3C/g%3E%3Cg fill-opacity='0.5'%3E%3Cpolygon fill='url(%23a)' points='100 30 0 0 200 0'/%3E%3Cpolygon fill='url(%23b)' points='100 100 0 130 0 100 200 100 200 130'/%3E%3C/g%3E%3C/svg%3E");
    background-color: #77CFEE;
    height: 100vh;
    width: 100vw;
}

#container {
    margin-top: 100px;
    margin-left: auto;
    margin-right: auto;
    /* border: 1px solid black; */
    width: 50%;
    background-color: white;
    border-radius: 30px;
    padding: 50px 0;
}

#form {
    /* border: 1px solid black; */
    width: 500px;
    margin: 0 auto;
}

#form p {
    text-align: left;
    margin: 20px 0 5px 0;
    font-size: 20px;
}

h1:first-letter {
    text-transform: capitalize;
}

input {
    width: 100%;
    height: 45px;
    padding-left: 10px;
    font-size: 16px
}

input::placeholder {
    text-indent: 10px;
}

#tip {
    margin-top: 30px;
    font-size: 20px;
}

button {
    margin-top: 25px;
    width: 50%;
    height: 45px;
    font-size: 20px;
    border-radius: 10px;
}
</style>
