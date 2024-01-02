<template>
    <auth-form title="Login" button-text="Submit" :submit-action="handleLoginOrRegister">
        <template v-slot:fields>
            <div class="input-group">
                <label for="zid">ZID:</label>
                <input type="text" id="zid" placeholder="ZID" v-model="zid" required />
            </div>
            <div class="input-group">
                <label for="password">Password:</label>
                <input type="password" id="password" placeholder="Password" v-model="password" required />
            </div>
            <div class="signup-link">
                Don't have an account? <router-link to="/signup">Sign Up</router-link>
            </div>
        </template>
    </auth-form>
</template>

<script>
import AuthForm from "../components/AuthForm.vue";

export default {
    components: { AuthForm },

    name: "LoginView",
    data() {
        return {
            zid: "",
            password: "",
        };
    },

    methods: {
        validateInput() {
            const zidRegex = /^z\d{7}$/;
            const passwordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,}$/;

            if (!zidRegex.test(this.zid)) {
                alert("ZID must start with 'z' followed by 7 digits.");
                return false;
            }

            if (!passwordRegex.test(this.password)) {
                alert(
                    "Password must be eight or more characters, including upper and lowercase letters and at least one number."
                );
                return false;
            }
            return true;
        },

        async handleLoginOrRegister() {
            if (this.validateInput()) {
                try {
                    const response = await this.loginUser();
                    
                    if (response.statusCode === 200) {
                        localStorage.setItem('token', response.token);
                        this.$router.push("/home");
                    } else {
                        
                        alert(response.message);
                    }
                } catch (error) {
                    console.error("Login error", error);
                    alert("An error occurred during login.");
                }
            }
        },

        async loginUser() {
            const username = `${this.zid}@ad.unsw.edu.au`;
            const password = this.password;

            try {
                const response = await this.$fetchReq("auth/login", "POST", {
                    username,
                    password,
                });
                return response;
            } catch (error) {
                console.error("Error in loginUser:", error);
                throw error; 
            }
        },
    },
};
</script>

<style scoped>

.input-group {
    margin-bottom: 15px;
    width: 80%; 
    height:fit-content;
}

.input-group input {
    width: 100%;
    padding: 12px; 
    margin-bottom: 20px; 
    border: 1px solid #ddd;
    border-radius: 4px;
    box-sizing: border-box;
}

.input-group label {
    display: block;
    margin-bottom: 5px; 
    margin-left: 10px; 
    text-align: left; 
    font-weight: bold; 
}

.signup-link {
    margin-top: 20px;
    text-align: center;
}
</style>