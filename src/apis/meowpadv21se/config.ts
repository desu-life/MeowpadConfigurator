import { IRgb } from "@/interface"
import { IKeyConfig, IKeyRTStatus, INormalKeyConfig, KeyState } from ".."
import { KeyCode } from "@/keycode"

export interface IKeyboard {
    hall_keys: IKeyConfig[]
    normal_keys: INormalKeyConfig[]
    jitters_elimination_time: number
    continuous_report: boolean
    kalman_filter: boolean
}

export interface ILighting {
    led_colors: IRgb[]
    lighting_mode: LightingMode
    max_brightness: number
    sleep_time: number
}

export interface IDebugValue {
    keys: IKeyRTStatus[]
    btns: KeyState[]
}


export enum LightingMode {
    Off = 0,
    Calibration = 1,
    Error = 2,

    Solid = 3,
    RainbowMode = 4,
    RainbowFlowMode = 5,
    PressRadianceMode = 6,

    BreatheGlowMode = 7,
    BreatheGlowAsyncMode = 8,

    RainDropMode = 9,
    TapToGlowMode = 10,
    SpeedLightMode = 11
}
