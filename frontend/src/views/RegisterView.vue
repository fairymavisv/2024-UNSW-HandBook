<template>
    <auth-form title="Register" button-text="Register" :submit-action="handleLoginOrRegister">
        <template v-slot:fields>
            <div class="input-group">
                <input type="email" placeholder="Email" v-model="email" required />
            </div>

            <div class="input-group">
                <input type="password" placeholder="Password" v-model="password" required />
            </div>

            <div class="input-group">
                <input type="password" placeholder="Confirm Password" v-model="repassword" required />
            </div>

            <div class="input-group">
                <input type="text" placeholder="Nickname" v-model="nickname" required />
            </div>

            <div class="input-group">
                <input type="text" placeholder="Verification Code" v-model="verificationCode" required />
            </div>
        </template>
    </auth-form>
</template>

<script>
import AuthForm from "../components/AuthForm.vue";
export default{
    components: { AuthForm },

    name:"RegisterView",

    data(){
        return {
            email:'',
            password: '',
            repassword:'',
        };
    },

    methods:{
        validateInput() {
            const emailRegex = /^z\d{7}@ad\.unsw\.edu\.au$/;
            const passwordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,}$/;

            if(!emailRegex.test(this.email)){
                alert("Invalid Email Format. Please use an email address that starts with 'z' followed by 7 digits and ends with '@ad.unsw.edu.au'");
                return false;
            }

            if (!passwordRegex.test(this.password)) {
                alert("Invalid Password Format. Password must contain at least one lowercase letter, one uppercase letter, one digit, and be at least 8 characters long.");
                return false;
            }

            if (this.password !== this.repassword) {
                alert("Passwords do not match. Please make sure the password and confirmation password are the same.");
                return false;
            }

            return true;
        },

        async handleLoginOrRegister() {
            if (this.validateInput()) {
                try {
                    const response = await this.RegisterUser();
                    
                    if (response.statusCode === 200) {
                        this.$router.push("/home");
                    } else {
                        
                        alert(response.message);
                    }
                } catch (error) {
                    console.error("Registeration error", error);
                    alert("An error occurred during login.");
                }
            }
        },

        async RegisterUser() {
            const username = this.email;
            const password = this.password;
            const confirmPassword = this.repassword

            const response = await this.$fetchReq('auth/register', 'POST', {username, password, confirmPassword});
            return response;
        },

        
    }
};
</script>

<style scoped>
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
</style>