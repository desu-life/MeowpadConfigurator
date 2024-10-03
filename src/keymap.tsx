import { IMixedKey } from "@/apis/meowboard/config";

export declare type KeyMapType =
  | "BasicKeys"
  | "ExtendedKeys"
  | "FunctionKeys"
  | "MediaKeys"
  | "MouseKeys";

export interface IKeyMap {
  key: JSX.Element;
  code: IMixedKey;
}

export interface IKeyMaps {
  type: KeyMapType;
  keys: IKeyMap[];
}

export const mapping: IKeyMaps[] = [
  {
    type: "BasicKeys",
    keys: [
      { key: <span>A</span>, code: { t: "Keyboard", c: 0x04 } },
      { key: <span>B</span>, code: { t: "Keyboard", c: 0x05 } },
      { key: <span>C</span>, code: { t: "Keyboard", c: 0x06 } },
      { key: <span>D</span>, code: { t: "Keyboard", c: 0x07 } },
      { key: <span>E</span>, code: { t: "Keyboard", c: 0x08 } },
      { key: <span>F</span>, code: { t: "Keyboard", c: 0x09 } },
      { key: <span>G</span>, code: { t: "Keyboard", c: 0x0a } },
      { key: <span>H</span>, code: { t: "Keyboard", c: 0x0b } },
      { key: <span>I</span>, code: { t: "Keyboard", c: 0x0c } },
      { key: <span>J</span>, code: { t: "Keyboard", c: 0x0d } },
      { key: <span>K</span>, code: { t: "Keyboard", c: 0x0e } },
      { key: <span>L</span>, code: { t: "Keyboard", c: 0x0f } },
      { key: <span>M</span>, code: { t: "Keyboard", c: 0x10 } },
      { key: <span>N</span>, code: { t: "Keyboard", c: 0x11 } },
      { key: <span>O</span>, code: { t: "Keyboard", c: 0x12 } },
      { key: <span>P</span>, code: { t: "Keyboard", c: 0x13 } },
      { key: <span>Q</span>, code: { t: "Keyboard", c: 0x14 } },
      { key: <span>R</span>, code: { t: "Keyboard", c: 0x15 } },
      { key: <span>S</span>, code: { t: "Keyboard", c: 0x16 } },
      { key: <span>T</span>, code: { t: "Keyboard", c: 0x17 } },
      { key: <span>U</span>, code: { t: "Keyboard", c: 0x18 } },
      { key: <span>V</span>, code: { t: "Keyboard", c: 0x19 } },
      { key: <span>W</span>, code: { t: "Keyboard", c: 0x1a } },
      { key: <span>X</span>, code: { t: "Keyboard", c: 0x1b } },
      { key: <span>Y</span>, code: { t: "Keyboard", c: 0x1c } },
      { key: <span>Z</span>, code: { t: "Keyboard", c: 0x1d } },
      { key: <span>1</span>, code: { t: "Keyboard", c: 0x1e } },
      { key: <span>2</span>, code: { t: "Keyboard", c: 0x1f } },
      { key: <span>3</span>, code: { t: "Keyboard", c: 0x20 } },
      { key: <span>4</span>, code: { t: "Keyboard", c: 0x21 } },
      { key: <span>5</span>, code: { t: "Keyboard", c: 0x22 } },
      { key: <span>6</span>, code: { t: "Keyboard", c: 0x23 } },
      { key: <span>7</span>, code: { t: "Keyboard", c: 0x24 } },
      { key: <span>8</span>, code: { t: "Keyboard", c: 0x25 } },
      { key: <span>9</span>, code: { t: "Keyboard", c: 0x26 } },
      { key: <span>0</span>, code: { t: "Keyboard", c: 0x27 } },
      { key: <span>-</span>, code: { t: "Keyboard", c: 0x2d } },
      { key: <span>=</span>, code: { t: "Keyboard", c: 0x2e } },
      { key: <span>[</span>, code: { t: "Keyboard", c: 0x2f } },
      { key: <span>]</span>, code: { t: "Keyboard", c: 0x30 } },
      { key: <span>\</span>, code: { t: "Keyboard", c: 0x31 } },
      { key: <span>#</span>, code: { t: "Keyboard", c: 0x32 } },
      { key: <span>;</span>, code: { t: "Keyboard", c: 0x33 } },
      { key: <span>'</span>, code: { t: "Keyboard", c: 0x34 } },
      { key: <span>`</span>, code: { t: "Keyboard", c: 0x35 } },
      { key: <span>,</span>, code: { t: "Keyboard", c: 0x36 } },
      { key: <span>.</span>, code: { t: "Keyboard", c: 0x37 } },
      { key: <span>/</span>, code: { t: "Keyboard", c: 0x38 } },
    ],
  },
  {
    type: "ExtendedKeys",
    keys: [
      { key: <span>Esc</span>, code: { t: "Keyboard", c: 0x29 } },
      { key: <span>Enter</span>, code: { t: "Keyboard", c: 0x28 } },
      { key: <span>Caps</span>, code: { t: "Keyboard", c: 0x39 } },
      { key: <span>Backspace</span>, code: { t: "Keyboard", c: 0x2a } },
      { key: <span>Tab</span>, code: { t: "Keyboard", c: 0x2b } },
      { key: <span>Space</span>, code: { t: "Keyboard", c: 0x2c } },
      { key: <span>Ins</span>, code: { t: "Keyboard", c: 0x49 } },
      { key: <span>Home</span>, code: { t: "Keyboard", c: 0x4a } },
      { key: <span>PgUp</span>, code: { t: "Keyboard", c: 0x4b } },
      { key: <span>PgDn</span>, code: { t: "Keyboard", c: 0x4e } },
      { key: <span>Del</span>, code: { t: "Keyboard", c: 0x4c } },
      { key: <span>End</span>, code: { t: "Keyboard", c: 0x4d } },
      { key: <span>LCtrl</span>, code: { t: "Keyboard", c: 0xe0 } },
      { key: <span>LShift</span>, code: { t: "Keyboard", c: 0xe1 } },
      { key: <span>LAlt</span>, code: { t: "Keyboard", c: 0xe2 } },
      { key: <span>LMeta</span>, code: { t: "Keyboard", c: 0xe3 } },
      { key: <span>RCtrl</span>, code: { t: "Keyboard", c: 0xe4 } },
      { key: <span>RShift</span>, code: { t: "Keyboard", c: 0xe5 } },
      { key: <span>RAlt</span>, code: { t: "Keyboard", c: 0xe6 } },
      { key: <span>RMeta</span>, code: { t: "Keyboard", c: 0xe7 } },
      { key: <span>PScreen</span>, code: { t: "Keyboard", c: 70 } },
      { key: <span>←</span>, code: { t: "Keyboard", c: 0x50 } },
      { key: <span>↓</span>, code: { t: "Keyboard", c: 0x51 } },
      { key: <span>↑</span>, code: { t: "Keyboard", c: 0x52 } },
      { key: <span>→</span>, code: { t: "Keyboard", c: 0x4f } },
      { key: <span>F1</span>, code: { t: "Keyboard", c: 0x3a } },
      { key: <span>F2</span>, code: { t: "Keyboard", c: 0x3b } },
      { key: <span>F3</span>, code: { t: "Keyboard", c: 0x3c } },
      { key: <span>F4</span>, code: { t: "Keyboard", c: 0x3d } },
      { key: <span>F5</span>, code: { t: "Keyboard", c: 0x3e } },
      { key: <span>F6</span>, code: { t: "Keyboard", c: 0x3f } },
      { key: <span>F7</span>, code: { t: "Keyboard", c: 0x40 } },
      { key: <span>F8</span>, code: { t: "Keyboard", c: 0x41 } },
      { key: <span>F9</span>, code: { t: "Keyboard", c: 0x42 } },
      { key: <span>F10</span>, code: { t: "Keyboard", c: 0x43 } },
      { key: <span>F11</span>, code: { t: "Keyboard", c: 0x44 } },
      { key: <span>F12</span>, code: { t: "Keyboard", c: 0x45 } },

      { key: <span>Num /</span>, code: { t: "Keyboard", c: 0x54 } },
      { key: <span>Num *</span>, code: { t: "Keyboard", c: 0x55 } },
      { key: <span>Num -</span>, code: { t: "Keyboard", c: 0x56 } },
      { key: <span>Num +</span>, code: { t: "Keyboard", c: 0x57 } },
      { key: <span>Num Enter</span>, code: { t: "Keyboard", c: 0x58 } },
      { key: <span>Num 1</span>, code: { t: "Keyboard", c: 0x59 } },
      { key: <span>Num 2</span>, code: { t: "Keyboard", c: 0x5a } },
      { key: <span>Num 3</span>, code: { t: "Keyboard", c: 0x5b } },
      { key: <span>Num 4</span>, code: { t: "Keyboard", c: 0x5c } },
      { key: <span>Num 5</span>, code: { t: "Keyboard", c: 0x5d } },
      { key: <span>Num 6</span>, code: { t: "Keyboard", c: 0x5e } },
      { key: <span>Num 7</span>, code: { t: "Keyboard", c: 0x5f } },
      { key: <span>Num 8</span>, code: { t: "Keyboard", c: 0x60 } },
      { key: <span>Num 9</span>, code: { t: "Keyboard", c: 0x61 } },
      { key: <span>Num 0</span>, code: { t: "Keyboard", c: 0x62 } },
      { key: <span>Num .</span>, code: { t: "Keyboard", c: 0x63 } },
      { key: <span>Num =</span>, code: { t: "Keyboard", c: 0x67 } },
      { key: <span>Num ,</span>, code: { t: "Keyboard", c: 0x85 } },
      { key: <span>Num (</span>, code: { t: "Keyboard", c: 0xb6 } },
      { key: <span>Num )</span>, code: { t: "Keyboard", c: 0xb7 } },

      { key: <span>IntlRo</span>, code: { t: "Keyboard", c: 0x87 } },
      { key: <span>IntlYen</span>, code: { t: "Keyboard", c: 0x7d } },
    ],
  },
  {
    type: "FunctionKeys",
    keys: [
        { key: <span>Fn</span>, code: { t: "Custom", c: 1 } },
        { key: <span>Lock Win</span>, code: { t: "Custom", c: 2 } }
    ],
  },
  {
    type: "MediaKeys",
    keys: [
        { key: <span>Next Track</span>, code: { t: "Media", c: 0xb5 } },
        { key: <span>Previous Track</span>, code: { t: "Media", c: 0xb6 } },
        { key: <span>Stop</span>, code: { t: "Media", c: 0xb7 } },
        { key: <span>Play/Pause</span>, code: { t: "Media", c: 0xcd } },
        { key: <span>Mute</span>, code: { t: "Media", c: 0xe2 } },
        { key: <span>Volume Up</span>, code: { t: "Media", c: 0xe9 } },
        { key: <span>Volume Down</span>, code: { t: "Media", c: 0xea } },
    ],
  },
  {
    type: "MouseKeys",
    keys: [
        { key: <span>Left</span>, code: { t: "Mouse", c: 1 } },
        { key: <span>Right</span>, code: { t: "Mouse", c: 2 } },
        { key: <span>Middle</span>, code: { t: "Mouse", c: 3 } }
    ],
  },
];


export function formatKey(key: IMixedKey) {
  for (let i = 0; i < mapping.length; i++) {
    for (let j = 0; j < mapping[i].keys.length; j++) {
      if (mapping[i].keys[j].code.t == key.t && mapping[i].keys[j].code.c == key.c) {
        return mapping[i].keys[j].key
      }
    }
  }
  // if (key.t == "Keyboard") {
  //   return mapping[key.c] ?? ''
  // } else if (key.t == "Mouse") {
  //   switch (key.c) {
  //     case 1:
  //       return "Left";
  //     case 2:
  //       return "Right";
  //     case 3:
  //       return "Middle";
  //   }
  // } else if (key.t == "Custom") {
  //   switch (key.c) {
  //     case 1:
  //       return "Fn";
  //     case 2:
  //       return "Lock Win";
  //   }
  // } else if (key.t == "Media") {
  //   switch (key.c) {
  //     case 0xb5:
  //       return "Next Track";
  //     case 0xb6:
  //       return "Previous Track";
  //     case 0xb7:
  //       return "Stop";
  //     case 0xcd:
  //       return "Play/Pause";
  //     case 0xe2:
  //       return "Mute";
  //     case 0xe9:
  //       return "Volume Up";
  //     case 0xea:
  //       return "Volume Down";
  //   }
  // }
  return <span>{ '' }</span>
}