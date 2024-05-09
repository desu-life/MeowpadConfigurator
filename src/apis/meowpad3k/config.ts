import { IRgb } from "@/interface"
import { IKeyConfig } from ".."

export interface IKeyboard {
    keys: IKeyConfig[]
    jitters_elimination_time: number
    continuous_report: boolean
    kalman_filter: boolean
}

export interface ILighting {
    led_colors: IRgb[]
    max_brightness: number
    sleep_time: number
}


export enum LightingMode {
    Off = 0,
    Calibration = 1,
    Error = 2,

    Solid = 3,
}
