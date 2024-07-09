<script setup lang="ts">
defineProps({
  width: {
    type: String,
    required: true
  }
})

</script>

<template>

  <div
      class="frame"
      :style="{width: width}"
  >
    <div
        class="key"
    >
      <slot>

      </slot>
    </div>
  </div>

</template>

<style scoped lang="scss">
.frame {
  .key {
    display: flex;
    align-items: center;
    margin: 2px;
    justify-content: center;
    border-radius: 8px;
    height: 58px;
    width: inherit - 4px;
    position: relative;
    background-color: #1A1A1A;

    transition: transform 0.2s ease-in-out;
  }

}
</style>