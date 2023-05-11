<template>
  <div class="container w-full">
    
    <div class="form-container">
      <div class="form-wrapper">
        <div class="blur-container">
          <div class="blur-overlay"></div>
          <div class="form-content">
            <h2 class="form-title">Login</h2>
            <input v-model="email" type="email" class=" text-slate-600" placeholder="Email" />
            <input v-model="password" type="password" class=" text-slate-600" placeholder="Password" />
            <button @click="login">Submit</button>
            <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
          </div>
        </div>
      </div>
    </div>
  
  </div>
</template>

<style>
.container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100vh;
  max-width: 100%;
  background: linear-gradient(to right, #810010, #812f6a);
  font-family: "Arial", sans-serif;
  color: #fff;
}

.form-container {
  flex: 2;
  display: flex;
  justify-content: center;
  align-items: center;
}

.form-wrapper {
  position: relative;
  width: 300px;
  height: 300px;
}

.blur-container {
  position: relative;
  width: 100%;
  height: 100%;
}

.blur-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  backdrop-filter: blur(8px);
  z-index: 1;
}

.form-content {
  position: relative;
  z-index: 2;
  padding: 40px;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}

.form-title {
  text-align: center;
  font-size: 24px;
  margin-bottom: 20px;
}

input {
  width: 100%;
  padding: 10px;
  margin-bottom: 10px;
  border-radius: 5px;
  border: none;
}

button {
  width: 100%;
  padding: 10px;
  background-color: #00a8e8;
  color: #fff;
  border-radius: 5px;
  border: none;
  cursor: pointer;
}

p.error {
  text-align: center;
  color: #ff0000;
  margin-top: 10px;
}
</style>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { initializeApp } from 'firebase/app';
import { getAuth, signInWithEmailAndPassword } from 'firebase/auth';
import 'firebase/auth';
import local_key from '../local_env.json';

const firebaseConfig = {
  apiKey: local_key.apiKey,
  authDomain: local_key.authDomain,
  projectId: local_key.projectId,
  storageBucket: local_key.storageBucket,
  messagingSenderId: local_key.messagingSenderId,
  appId: local_key.appId,
  measurementId: local_key.measurementId
};

const fire_app = initializeApp(firebaseConfig);
const auth = getAuth();

export default defineComponent({
  name: 'LoginComponent',
  data() {
    return {
      email: '',
      password: '',
      errorMessage: '', // Add error message variable
    };
  },
  methods: {
    async login() {
      try {
        const { email, password } = this;
        console.log(email);
        const userCredential = await signInWithEmailAndPassword(auth, email, password);
        const { user } = userCredential;
        const { uid, email: userEmail } = user;
        this.$emit('login-success', { uid, email: userEmail });
      } catch (error: any) {
        // Handle login error
        console.error(error);
        if (error.code === 'auth/user-not-found') {
          this.errorMessage = 'User not found. Please check your email or register.';
        } else if (error.code === 'auth/wrong-password') {
          this.errorMessage = 'Incorrect password. Please try again.';
        } else {
          this.errorMessage = 'Login failed. Please try again later.';
        }
      }
    },
  },
});
</script>
