<script setup lang="ts">
defineProps({
  keyStr: {
    type: String,
    required: true
  }
})

const hallValue = defineModel("hallValue", { type: Number, default: 0 });
// const isSelected = ref(false);
const hallValuePercentage = defineModel("hallValuePercentage", { type: Number, default: 0 });
const isPressed = defineModel("isPressed", { type: Boolean, default: false });
function isActive() {
  return hallValue.value! > 0
}


</script>

<template>
  <div class="key-debug"
    :class="{
        pressed: isPressed,
    }"
  >
      <div class="label-frame">
        <div class="key-debug-text">
          {{ keyStr }}
        </div>
        <div class="key-hall">
          {{hallValue}}
        </div>
      </div>
      <div v-if="isActive()" class="percentage-frame">
        <div class="percentage" :style="{height: hallValuePercentage!.toString() + '%'}"></div>
      </div>
  </div>
</template>

<style scoped lang="scss">

.key-debug {
  display: flex;
  flex-direction: row;
  align-items: flex-end;
  justify-content: flex-start;

  position: relative;

  padding: 8px;

  //height: inherit;
  width: 100%;

  //background-color: #33a06f;

  border-radius: inherit;

  &.pressed {
    background-color: rgba(251, 255, 19, 0.37)
  }

  .label-frame {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: flex-start;
    flex: 1;

    .key-debug-text {
      color: rgba(255, 255, 255, 0.2);
    }

    .key-hall {
      font-size: 12px;
    }
  }

  .percentage-frame {
    width: 5px;
    height: 40px;
    //background-color: #33a06f;

    border: 1px solid rgba(78, 78, 78, 0.2);

    display: flex;
    flex-direction: column;
    justify-content: flex-end;

    .percentage {
      width: 5px;
      background-color: rgba(255, 0, 0, 0.4);
    }
  }

}

</style>