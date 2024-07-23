import { IRgb } from "@/interface"
import { IKeyConfig } from ".."
import { KeyCode } from "@/keycode"

export interface IKeyConfigBoard {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    release_dead_zone: number
    rt_enabled: boolean
}


export interface IKeyboard {
    keys: IKeyConfigBoard[]
    normal_layer: KeyCode[]
    fn_layer: KeyCode[]
    jitters_elimination_time: number
    continuous_report: boolean
    hall_filter: number
    max_brightness: number
}
