import { initializeApp } from "firebase/app";
import { getAuth } from "firebase/auth";

const firebaseConfig = {
  apiKey: "AIzaSyCbZ7ysVjPP3FXTh6krPbHKtU-xmRTrXg0",
  authDomain: "occ-firebase-login-aa426.firebaseapp.com",
  projectId: "occ-firebase-login-aa426",
  storageBucket: "occ-firebase-login-aa426.appspot.com",
  messagingSenderId: "836226373434",
  appId: "1:836226373434:web:871a04f007032f49bbe203",
  measurementId: "G-L9DJ2YF43K"
};

const app = initializeApp(firebaseConfig);

const auth = getAuth(app);

export { auth };