<script lang="ts">
  let classNames: string = '';
  export { classNames as class };
</script>

<div class="dot-flashing {classNames}" />

<style>
  .dot-flashing {
    position: relative;
    width: 10px;
    height: 10px;
    border-radius: 5px;
    background-color: #387ff7;
    color: #387ff7;
    -webkit-animation: dot-flashing 1s infinite linear alternate;
    animation: dot-flashing 1s infinite linear alternate;
    -webkit-animation-delay: 0.5s;
    animation-delay: 0.5s;
  }
  .dot-flashing::before,
  .dot-flashing::after {
    content: '';
    display: inline-block;
    position: absolute;
    top: 0;
  }
  .dot-flashing::before {
    left: -15px;
    width: 10px;
    height: 10px;
    border-radius: 5px;
    background-color: #387ff7;
    color: #387ff7;
    -webkit-animation: dot-flashing 1s infinite alternate;
    animation: dot-flashing 1s infinite alternate;
    -webkit-animation-delay: 0s;
    animation-delay: 0s;
  }
  .dot-flashing::after {
    left: 15px;
    width: 10px;
    height: 10px;
    border-radius: 5px;
    background-color: #387ff7;
    color: #387ff7;
    -webkit-animation: dot-flashing 1s infinite alternate;
    animation: dot-flashing 1s infinite alternate;
    -webkit-animation-delay: 1s;
    animation-delay: 1s;
  }

  @-webkit-keyframes dot-flashing {
    0% {
      background-color: #387ff7;
    }
    50%,
    100% {
      background-color: rgba(152, 128, 255, 0.2);
    }
  }

  @keyframes dot-flashing {
    0% {
      background-color: #387ff7;
    }
    50%,
    100% {
      background-color: rgba(152, 128, 255, 0.2);
    }
  }
</style>
