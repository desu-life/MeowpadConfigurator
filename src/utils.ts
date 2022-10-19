import { IRgb } from '@/interface';

function pad2(c) {
    return c.length == 1 ? '0' + c : '' + c;
}

export function Hex2Rgb(hex: string): IRgb {
    var match
    if (match = /^#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})$/.exec(hex)) {
        return {
            red: parseInt(match[1], 16),
            green: parseInt(match[2], 16),
            blue: parseInt(match[3], 16),
        };
    } else {
        return {
            red: 0,
            green: 0,
            blue: 0,
        };
    }
}

export function Rgb2Hex(color: IRgb): string {
    var hex = [
        pad2(Math.round(color.red).toString(16)),
        pad2(Math.round(color.green).toString(16)),
        pad2(Math.round(color.blue).toString(16))
    ];

    return "#" + hex.join("").toUpperCase();
}