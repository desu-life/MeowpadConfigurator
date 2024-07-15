import { IRgb } from "@/interface";
import { KeyCode } from "@/keycode";
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'
import * as api from '@/apis/api'

export declare type Error = 'DeviceDisconnected' | 'DeviceNotFound' | 'Network' | 'Meowpad' | 'Iap';
export declare type DeviceName = 'Meowpad' | 'Meowpad SE v2';

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
    name: DeviceName
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

export interface IKeyHallConfig {
    adc_min: number
    adc_max: number
    hall_middle: number
}


export enum KeyState {
    Released = 0,
    Pressed = 1,
    Calibrating = 2
}

