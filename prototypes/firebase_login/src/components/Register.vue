<script lang="ts">
    
    import {Component, Vue} from "vue-property-decorator";
    // Import the functions you need from the SDKs you need
    import { initializeApp } from "firebase/app";
    import { getAnalytics } from "firebase/analytics";
    import firebase from 'firebase';


    @Component({

        name: "register",

    })
    
    export default class Register extends Vue {
        email:string = "";
        password:string = "";

        registerNewUser():void{
            firebase.auth().createUserWithEmailAndPassword(this.email, this.password).then(res => {
                console.log(res);
                
                let token:string = res.user.ma;
                let userId:String = res.user.uid;

                localStorage.setItem('token', token);
                localStorage.setItem('userId', token);
                
                this.$router.push('/profile');

                
            }).catch(function(error)) {
                // handle erros here
                var errorCode = error.code;
                var errorMessage = error.message;
                alert(errorMessage);
                // ...
            }
        }

    }

</script>

<template>
    
    <div>
        <h1>Register user</h1>
        <input type="email" placeholder="email" v-model="email">
        <input type="password" placeholder="password" v-model="password">
        <button @click="registerNewUser">Register</button>
    </div>
</template>

<style lang="scss" scoped>

</style>