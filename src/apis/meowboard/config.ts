import { IRgb } from "@/interface"
import { IKeyConfig } from ".."

export interface IKeyConfigBoard {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    release_key_data: number
}


export interface IKeyboard {
    keys: IKeyConfigBoard[]
    jitters_elimination_time: number
    continuous_report: boolean
    kalman_filter: boolean
    enable_hs: boolean
}
