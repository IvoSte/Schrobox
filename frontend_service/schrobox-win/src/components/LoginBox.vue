<template>
    <div class="container">
    <h1 class="title">
        Authentication
    </h1>
    <h2 class="subtitle">
        Please use the provided credentials.
    </h2>
        <form @submit.prevent="handleSubmit">
            <div class="username-entry">
                <span class=" has-text-black" >Username</span>
                <input class="input py-4" type="text" v-model="username" name="username" placeholder="Username"/>
            </div>
            <div class="password-entry">
                <span class=" has-text-black" >Password</span>
                <input class="input py-4" type="password" v-model="password" name="password" placeholder="*************"/>
            </div>
            <div class="button-block">
                <button class="login-button button is-xl">Login</button>
                <button class="create-account-button button is-xl">Create Account</button>
            </div>
        </form>
    </div>
</template>

<script>
import { ref } from "vue";
import { useStore } from "vuex";

  export default {
      name: "loginBox",
      setup() {
        const store = useStore();
//        const router = useRouter();
        const username = ref("");
        const password = ref("");
        const submitted = ref(false);

        const handleSubmit = async function () {
        submitted.value = true;
        if (username.value && password.value) {
            console.log("Attempting login...");

            // TODO: Catch errors
            store
            .dispatch("login", {
                username: username.value,
                password: password.value,
            })
//            .then(() => router.push({ name: "sheet" }));
        }
        };

        return {
            username: username,
            password: password,
            submitted: submitted,
            handleSubmit: handleSubmit,
        }
      }
    }
</script>

<style lang="scss" scoped>
@import '../assets/scss/globals.scss';
.title{
    font-size: 3rem;
    color: $prim-dark;
}
.subtitle {
    color: $prim-dark;
    margin-bottom: 4rem;
}
.username-entry {
    margin-bottom: 1rem;
}
.password-entry {
    margin-bottom: 2rem;
}
.button-block {
    display: flex;
    flex-direction: row;
}
.button{
    background-color: $prim-dark;
    color: $prim-light;
    border-radius: 5px;
    border-style: solid;
    border-color: $prim-dark;
}
.login-button {
    margin-right: 1rem;
}
.container{
    background-color: $prim-light;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    border-radius: 20px;
    border-style: solid;
    border-width: 5px;
    border-color: $prim-dark;
    padding: 2rem;
    padding-top: 5rem;
    padding-bottom: 5rem;
}

</style>