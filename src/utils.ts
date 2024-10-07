import { useI18n } from "vue-i18n";
import { IError } from "./apis";
import { KeyCode, mapping } from "./keycode";
import { IRgb } from "./interface";
import { IMixedKey } from "@/apis";
import { mapping as keymap } from "@/keymap";

function pad2(c) {
  return c.length == 1 ? "0" + c : "" + c;
}



export function splitArray<T>(array: T[], sizes: number[]): T[][] {
  let result: T[][] = [];
  let index = 0;

  for (let size of sizes) {
    let subArray = array.slice(index, index + size);
    result.push(subArray);
    index += size;
  }

  return result;
}

export function most(arr) {
  let obj = arr.reduce((p, n) => (
    p[n]++ || (p[n] = 1),
    (p.max = p.max >= p[n] ? p.max : p[n]),
    (p.key = p.max > p[n] ? p.key : n), p), {})
  return obj.key
}

export function getErrorMsg(t, e: IError): string {
  if (e.data) {
    return t(e.data);
  }
  if (e.type) {
    return t(e.type);
  }
  console.error(e);
  return e.toString();
}

export function formatKeys(keycodes: KeyCode[]) {
  const keys = keycodes
    .filter((k) => k != KeyCode.None)
    .map((k) => KeyCode[k])
    .join(" + ");
  return keys === "" ? "无" : keys;
}

export function Hex2Rgb(hex: string): IRgb {
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

export function time_2_str() {
  const date = new Date();
  const year = date.getFullYear();
  const month = date.getMonth() + 1;
  const day = date.getDate();
  const hours = date.getHours();
  const minutes = date.getMinutes();

  const dateString = `${year}-${month}-${day}-${hours}-${minutes}`
  return dateString
}