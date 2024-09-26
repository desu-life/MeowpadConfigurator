import { IRgb } from "@/interface"
import { IKeyConfig } from ".."
import { KeyCode } from "@/keycode"

export declare type KeyType = 'None' | 'Keyboard' | 'Custom' | 'Mouse' | 'Media';

export interface IKeyConfigBoard {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    release_dead_zone: number
    rt_enabled: boolean
}

export interface IMixedKey {
    t: KeyType,
    c: KeyCode | number
}


export interface IKeyboard {
    keys: IKeyConfigBoard[]
    normal_layer: IMixedKey[]
    fn_layer: IMixedKey[]
    jitters_elimination_time: number
    high_reportrate: boolean
    hall_filter: number
    max_brightness: number
}
