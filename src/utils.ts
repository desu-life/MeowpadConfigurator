import { useI18n } from "vue-i18n";
import { IError } from "./apis";
import { KeyCode } from "./keycode";
import { IRgb } from "./interface";

function pad2(c) {
  return c.length == 1 ? "0" + c : "" + c;
}

export function getErrorMsg(t, e: IError): string {
  if (e.data) {
    return t(e.data);
  }
  return t(e.type) || "未知错误";
}

export function formatKeys(keycodes: KeyCode[]) {
  const keys = keycodes
    .filter((k) => k != KeyCode.None)
    .map((k) => KeyCode[k])
    .join(" + ");
  return keys === "" ? "无" : keys;
}

export function Hex2Rgb(hex: string): IRgb {
  console.log(hex)
  var match;
  if (
    (match = /^#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})$/.exec(hex))
  ) {
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
    pad2(Math.round(color.blue).toString(16)),
  ];

  return "#" + hex.join("").toUpperCase();
}

export function IsModifierKey(key: KeyCode): boolean {
  return KeyCode.LCtrl <= key && key <= KeyCode.RGui;
}

export function compareArray<T>(a1: T[], a2: T[]): boolean {
  if (a1 === a2) return true;
  if ((!a1 && a2) || (a1 && !a2)) return false;
  if (a1.length !== a2.length) return false;
  for (var i = 0, n = a1.length; i < n; i++) {
    if (a1[i] !== a2[i]) return false;
  }
  return true;
}

export function formatms(value: number | null) {
  if (value === null) return "";
  return `${value} ms`;
}

export function parsems(value: string) {
  if (value === "") return null;
  return parseInt(value);
}
