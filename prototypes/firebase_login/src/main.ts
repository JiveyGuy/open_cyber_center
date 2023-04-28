// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
import { getAnalytics } from "firebase/analytics";
import { getAuth, onAuthStateChanged, signInWithRedirect, GoogleAuthProvider} from 'firebase/auth';
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseConfig = {
  apiKey: "AIzaSyCbZ7ysVjPP3FXTh6krPbHKtU-xmRTrXg0",
  authDomain: "occ-firebase-login-aa426.firebaseapp.com",
  projectId: "occ-firebase-login-aa426",
  storageBucket: "occ-firebase-login-aa426.appspot.com",
  messagingSenderId: "836226373434",
  appId: "1:836226373434:web:51ae3748fafb5260bbe203",
  measurementId: "G-WP2D2HMERH"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);

const auth = getAuth();
const button = document.querySelector('button')

onAuthStateChanged(auth, user => {
    if(user == null) {return; }
    console.log(user);
})

button?.addEventListener('click', clickEvent => {
    signInWithRedirect(auth, new GoogleAuthProvider());
});