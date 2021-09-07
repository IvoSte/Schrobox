<template>
<div class="login">
  <section class="hero is-primary">
    <div class="hero-body">
      <div class="container">
        <h1 class="title">
            Authentication
        </h1>
        <h2 class="subtitle">
          Please use the provided credentials.
        </h2>
        <form action="/login_credentials.php">
            <div class="username">
            Username:
            </div>
            <div class="username-entry">
            <input type="text">
            </div>
            <div class="">
            Password:
            </div>
            <div class="password-entry">
            <input type="text">
            </div>
            <div class="button-block">
            <button class="button is-xl is-dark">Login</button>
            </div>
        </form>
      </div>
    </div>
  </section>
  <EventsList />
</div>
</template>
<script>
import EventsList from '../components/EventsList';
export default {
  name: 'home',
  components: {
    EventsList
  }
}
</script>
<style lang="scss" scoped>
  .hero {    
    text-align: center;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    height: 400px;
  }
  .hero-body .title {
    text-shadow: 4px 4px 4px rgba(0, 0, 0, 0.6);
    padding: 40px 0 20px 0;
    font-size: 60px;
  }
  .subtitle {
    text-shadow: 4px 4px 4px rgba(0, 0, 0, 0.7);
    font-size: 30px;
  }
  .button-block {
    text-align: center;
    margin-left: auto;
    margin-right: auto;
    width: 100%;
    position: absolute;
    bottom: -150px;
    .button {
      margin-right: 50px;
      padding-left: 50px;
      padding-right: 50px;
    }
    .welcome {
      width: 400px;
      padding: 10px;
      margin-left: auto;
      margin-right: auto;
    }
  }
  .is-xl {
    font-size: 1.7rem;
  }
</style>