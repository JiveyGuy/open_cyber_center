<script lang="ts">
    
    import {Component, Vue} from "vue-property-decorator";


    @Component({

        name: "profile",

    })
    
    export default class Profile extends Vue {
        userId:string = '';

        created():void{
            console.log("id", this.userId);
            this.userId = localStorage.getItem('userId');

            if(databaseID != this.userId){
                logout();
            }
        }
    }

</script>

<template>
    
    <div>
        <h1>Hello - {{ userId }}</h1>
    </div>
</template>



<style lang="scss" scoped>

</style>