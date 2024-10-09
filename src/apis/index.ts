import { IRgb } from "@/interface";
import { KeyCode } from "@/keycode";
import * as api4k from '@/apis/meowpad4k/api'
import * as api3k from '@/apis/meowpad3k/api'
import * as api from '@/apis/api'

export declare type Error = 'DeviceDisconnected' | 'DeviceNotFound' | 'Network' | 'Meowpad' | 'Iap';
export declare type DeviceName = 'Meowpad' | 'Meowpad SE v2' | 'Pure64';
export declare type KeyType = 'None' | 'Keyboard' | 'Custom' | 'Mouse' | 'Media';

export interface IError {
    type: Error
    data?: any
}

export interface IDeviceStatus {
    key: boolean
    light?: boolean
    hall: boolean
    enabled: boolean
}

export interface IDeviceInfo {
    name: DeviceName
    version: string
}

export interface IVersion {
    version: string
    infomation: IVersionInfo
}
export interface IVersionInfo {
    notes: string
    date: string
    platforms: IVersionPlatforms
}
export interface IVersionPlatforms {
    'macos-app'?: IVersionPlatform
    'linux-appimage'?: IVersionPlatform
    'windows-x86_64'?: IVersionPlatform
}
export interface IVersionPlatform {
    hash: string
    url: string
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

export interface IHidDeviceInfo {
    path: number[]
    vendor_id: number
    product_id: number
    interface_number: number
    device_name: DeviceName
    firmware_version: string
    serial_number?: string
}

export enum KeyState {
    Pressed = 0,
    Released = 1,
    Calibrating = 2
}

export interface IMixedKey {
    t: KeyType,
    c?: KeyCode | number
}



export interface IDevicePreset {
    id: string
    name: string
    device: IDevicePresetInfo
    config: IPresetConfig
}


export interface IDevicePresetInfo {
    device_name: DeviceName
    serial_number?: string
}


export interface IPresetConfig {
    key_layers?: IKeyPresetLayer[]
    key_configs?: IKeyPresetConfig[]
}

export interface IKeyPresetLayer {
    keys: IMixedKey[]
}


export interface IKeyPresetConfig {
    press_percentage: number
    release_percentage: number
    dead_zone: number
    release_dead_zone: number
    rt_enabled?: boolean
}