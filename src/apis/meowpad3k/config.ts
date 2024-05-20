import { IRgb } from "@/interface"
import { IKeyConfig, IKeyRTStatus, KeyState } from ".."
import { KeyCode } from "@/keycode"

export interface IKeyboard {
    keys: IKeyConfig[]
    side_btn: KeyCode[]
    jitters_elimination_time: number
    continuous_report: boolean
    kalman_filter: boolean
}

export interface ILighting {
    led_colors: IRgb[]
    max_brightness: number
    sleep_time: number
}

export interface IDebugValue {
    key: IKeyRTStatus[]
    btn: KeyState
}

export enum LightingMode {
    Off = 0,
    Calibration = 1,
    Error = 2,

    Solid = 3,
}
