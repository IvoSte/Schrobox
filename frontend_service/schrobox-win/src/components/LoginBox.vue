<template>
    <div class="container">
    <h1 class="title">
        Authentication
    </h1>
    <h2 class="subtitle">
        Please use the provided credentials.
    </h2>
        <form @submit.prevent="handleSubmit">
            <div class="username">
            Username:
            </div>
            <div class="username-entry">
            <input type="text" v-model="username" name="username" />
            </div>
            <div class="">
            Password:
            </div>
            <div class="password-entry">
            <input type="password" v-model="password" name="password" />
            </div>
            <div class="button-block">
            <button class="button is-xl is-dark">Login</button>
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