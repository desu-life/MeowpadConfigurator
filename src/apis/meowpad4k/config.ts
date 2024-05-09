import { IRgb } from "@/interface"
import { IKeyConfig } from ".."

export interface IKeyboard {
    keys: IKeyConfig[]
    jitters_elimination_time: number
    continuous_report: boolean
    kalman_filter: boolean
    enable_hs: boolean
}

export interface ILighting {
    led_colors: IRgb[]
    lighting_mode: LightingMode
    lighting_mode_sleep: LightingMode
    max_brightness: number
    sleep_time: number
    
    // rainbow_flow_mode
    rainbow_flow_speed: number
    color_change_rate: number
    is_flow_delay: boolean

    // rainbow_mode
    rainbow_speed: number

    // breathing_mode
    breathing_speed: number
    max_keep_time: number
    min_keep_time: number
    breaths_before_color_switch: number

    // rain_drop_mode
    rain_drop_speed: number
    random_rain_chance: number

    // tap_to_glow_mode
    tap_to_glow_speed: number
    max_lum_freeze_time: number
    change_color_when_pressed: boolean
    random_color_mode: boolean

    // speed_light_mode
    speed_light_mode_speed: number,
    attenuation_speed: number,
    increase_difficulty: number,
    low_speed_color: IRgb,
    high_speed_color: IRgb,
}


export enum LightingMode {
    Off = 0,
    Debug = 1,
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
