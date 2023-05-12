<template>
  <div class="bg-gray-900 text-white min-h-screen min-w-full">
    <div class="container mx-auto py-10 min-w-full">
      <h2 class="text-3xl font-bold mb-4">Admin Dashboard</h2>
      <div class="flex">
        <div class="w-1/2 pr-10">
          <div v-if="users.length > 0">
            <div>
              <h3 class="text-xl font-bold mb-2">Users</h3>
              <div class="grid grid-cols-3 gap-4">
                <div v-for="user in users" :key="user.id" class="flex flex-col items-center">
                  <img src="../assets/vue.svg" alt="User Image" class="w-24 h-24 mb-2 rounded-full">
                  <span>{{ user.name }}</span>
                </div>
              </div>
            </div>
          </div>
          <div>
            <button @click="fetchUsers" class="px-4 py-2 rounded bg-blue-500 text-white">Reload Users</button>
          </div>
          <div v-if="errorMessage" class="text-red-500">{{ errorMessage }}</div>
        </div>

        <div class="w-1/2 pl-10">
          <div class="mt-4">
            <h3 class="text-xl font-bold mb-2">Add User</h3>
            <input v-model="newUserName" type="text" placeholder="User Name" class="mr-2 p-2 rounded border border-gray-300 text-slate-700">
            <button @click="addUser" class="px-4 py-2 mt-3 rounded bg-blue-500 text-slate-900">Add</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>


<script lang="ts">
import { defineComponent, ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { initializeApp } from 'firebase/app';
import { getAuth, signInWithEmailAndPassword } from 'firebase/auth';
import 'firebase/auth';
import local_key from '../local_env.json';
import { getDatabase, ref as dbRef, onValue } from 'firebase/database';


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
// const auth = getAuth();
const db = getDatabase();


interface User {
  id: string;
  name: string;
}

export default defineComponent({
  name: 'AdminDashboard',
  setup() {

    // function delay(ms: number) {
    //   return new Promise( resolve => setTimeout(resolve, ms) );
    // }
    // setup firebade connection like in firebase_login_jivey
    const users = ref<User[]>([]);
    const errorMessage = ref('');

    const newUserName = ref('');

    const fetchUsers = async () => {
      try {
        const usersRef = dbRef(db, 'users');
        //listener for 'value' event to get the initial user data andy changes
        onValue(usersRef, (snapshot) => {
          const usersData = snapshot.val();
          const usersArray = []; 

          for(const userId in usersData){
            if(Object.prototype.hasOwnProperty.call(usersData, userId)){
              const user = {
                id: userId,
                name: usersData[userId].name,
              };
              usersArray.push(user);
            }
          }
          users.value = usersArray;
          console.log(usersArray);
        });    
        // get an array [] of user IDs or name from firebase and pass it to users.value

        // Get list of users from firebase 


        users.value = [{ id: 1, name: "Adam" }, { id: 1, name: "Amy" }, { id: 1, name: "Ark" }];
      } catch (error) {
        errorMessage.value = 'Failed to fetch users.';
        console.error(error);
      }
    };

    const addUser = async () => {
      try {
        if (newUserName.value) {

          // Pass newUserName.value to a method that will insert a new username and password
          //  for this test make the password the same for all const pass = "helloword"

          // await invoke('add_user', { name: newUserName.value }); // Invoke Rust command to add a user
          newUserName.value = '';
          await delay(5000);
          await fetchUsers(); // Refresh the list of users
        }
      } catch (error) {
        errorMessage.value = 'Failed to add user.';
        console.error(error);
      }
    };

    fetchUsers();

    return {
      users,
      newUserName,
      addUser,
      fetchUsers,
      errorMessage,
    };
  },
});
</script>

<style>
.container {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
}
</style>