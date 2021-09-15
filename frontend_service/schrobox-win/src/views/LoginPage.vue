<template>
    <div class="container">    
        <img class="logo" src="../assets/images/Schrobox_logo.png" />
      <LoginBox/>
    </div>
</template>

<script>
import LoginBox from '../components/LoginBox';

export default {
  name: 'loginpage',
  components: {
    LoginBox
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
  .container {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
  }
  .logo {
    margin-bottom: 2rem;
    width: 30%;
  }
</style>