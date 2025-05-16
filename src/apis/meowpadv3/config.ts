import { IRgb } from "@/interface"
import { IKeyConfig, IMixedKey } from ".."
import { KeyCode } from "@/keycode"


export interface IKeyConfigV3 {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    release_dead_zone: number
    rt_enabled: boolean
}

export interface ISOCDKeyPairs {
    key1: number
    key2: number
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
}
