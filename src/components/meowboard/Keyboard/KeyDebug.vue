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
const keyShowMode = defineModel("keyShowMode", { type: Number, default: 0 });
const totalDistance = defineModel("totalDistance", { type: Number, default: 4.0 });
function isActive() {
  return hallValue.value! > 0
}

function format_precentage(value: number) {
  let precentage = value / 100.0
  let res = 0
  if (keyShowMode.value === 0) {
    res = precentage * 100
  } else if (keyShowMode.value === 1) {
    res = precentage * totalDistance.value
  }

  if (res === 100) return res.toString()
  return res.toFixed(2)
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
          {{ format_precentage(hallValuePercentage) }}
        </div>
        <div class="key-hall">
          {{hallValue}}
        </div>
      </div>
      <div v-if="keyShowMode === 1" class="percentage-frame">
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
      color: rgba(255, 255, 255, 0.555);
      font-size: 13px;
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