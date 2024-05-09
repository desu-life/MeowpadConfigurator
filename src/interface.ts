export interface IKeymap {
    name?:  string;
    width: number;
    index?: number;
}

export interface IRgb {
    red: number
    green: number
    blue: number
}

export enum Toggle {
    On = 1,
    Off = 0,
}