<script lang="ts">
    
import { FirebaseError } from "firebase/app";
import firebase from 'firebase';
import {Component, Vue} from "vue-property-decorator";


    @Component({

        name: "login",

    })
    
    export default class Login extends Vue {
        email:string = '';
        password:string = '';

        LoginUser():void{
            FirebaseError.auth().signInWithEmailAndPassword(this.email, this.password).then(res => {
                let token:string = res.user.ma;
                let userId:String = res.user.uid;

                localStorage.setItem('token', token);
                localStorage.setItem('userId', token);
                
                this.$router.push('/profile');

            }).catch(function(error)) {

                var errorCode = error.code;
                var errorMessage = error.message;
                alert(errorMessage);

            });
        }
    }

</script>

<template>
    
    <div>
        <h1>Login user</h1>
        <input type="email" placeholder="email" v-model="email">
        <input type="password" placeholder="password" v-model="password">
        <button @click="LoginUser">Login</button>
    </div>
</template>



<style lang="scss" scoped>

</style>