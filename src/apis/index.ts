import { IRgb } from "@/interface";
import { KeyCode } from "@/keycode";

export declare type Error = 'DeviceDissconnected' | 'DeviceNotFound' | 'Network' | 'Meowpad' | 'Iap';

export interface IError {
    type: Error
    data?: any
}

export interface IDeviceStatus {
    key: boolean
    light: boolean
    hall: boolean
    enabled: boolean
}

export interface IDeviceInfo {
    name: string
    version: string
}

export interface IVersion {
    configurator_version: string
    download_url: string
    v2_starter_edition_latest_firmware_version: string
    v2_starter_edition_firmware_download_url: string
    v2_standard_edition_latest_firmware_version: string
    v2_standard_edition_firmware_download_url: string
}

export interface IKeyRTStatus {
    adc_value: number
    linear_value: number
    press_percentage: number
    key_state: KeyState
}

export interface IKeyConfig {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    key_data: KeyCode[]
}


export enum KeyState {
    Pressed = 0,
    Released = 1,
    Calibrating = 2
}

