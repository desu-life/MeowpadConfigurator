import { IRgb } from "@/interface"
import { IKeyConfig, IMixedKey, ISOCDKeyPairs } from ".."
import { KeyCode } from "@/keycode"


export interface IKeyConfigV3 {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    release_dead_zone: number
    rt_enabled: boolean
}


export interface IKeyboard {
    keys: IKeyConfigV3[]
    layer: IMixedKey[][]
    jitters_elimination_time: number
    high_reportrate: boolean
    key_proof: boolean
    auto_calibration: boolean
    hall_filter: number
    max_brightness: number
    led_color: IRgb
    socd_key_pairs: ISOCDKeyPairs[]
    led_mode: LightingMode
    sleep_timeout: number
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
}
