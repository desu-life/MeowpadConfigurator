import { IRgb } from "@/interface"
import { IKeyConfig } from ".."
import { KeyCode } from "@/keycode"

export interface IKeyConfigBoard {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    release_key_data: number
}


export interface IKeyboard {
    keys: IKeyConfigBoard[]
    normal_layer: KeyCode[]
    fn_layer: KeyCode[]
    jitters_elimination_time: number
    continuous_report: boolean
    kalman_filter: boolean
    enable_hs: boolean
}
