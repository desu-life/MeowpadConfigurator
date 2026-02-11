# WASM WebHID Integration

This directory contains the WebAssembly integration that enables the MeowpadConfigurator to run in web browsers using WebHID API.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                 Vue Components                       │
└─────────────────┬───────────────────────────────────┘
                  │
    ┌─────────────▼────────────────┐
    │  Unified API (src/wasm/)     │
    │  - Automatic env detection   │
    │  - Pure64, MeowpadV3, etc    │
    └─────┬──────────────┬─────────┘
          │              │
    ┌─────▼─────┐  ┌─────▼──────┐
    │   Tauri   │  │    WASM    │
    │  invoke() │  │  (WebHID)  │
    └───────────┘  └─────┬──────┘
                         │
                  ┌──────▼──────┐
                  │  WebHID API │
                  │  (Browser)  │
                  └─────────────┘
```

## Files

### Core Files
- **environment.ts** - Environment detection (Tauri vs Web)
- **loader.ts** - WASM module loading and initialization
- **index.ts** - Main export

### Device APIs
- **pure64-wasm.ts** - Pure64 WASM API wrapper
- **pure64/index.ts** - Pure64 unified API (Tauri + WASM)

### Build
- **../scripts/build-wasm.sh** - Build script for WASM module
- **../src-wasm/** - Rust source code compiled to WASM

## Usage

### 1. Initialize (App Startup)

```typescript
import { initialize, getRuntimeEnvironment } from '@/wasm';

// At app startup
async function initApp() {
  const env = getRuntimeEnvironment();
  console.log('Running in:', env); // 'tauri' or 'webhid'
  
  if (env === 'webhid') {
    await initialize(); // Load WASM module
  }
}
```

### 2. Use Unified API

```typescript
import * as Pure64 from '@/wasm/pure64';

// Same code works in both Tauri and Web!
async function connectDevice() {
  await Pure64.initialize();
  const connected = await Pure64.connect();
  
  if (connected) {
    const info = await Pure64.get_device_info();
    console.log('Device:', info.name, info.version);
  }
}
```

### 3. Device Operations

```typescript
// Get device status
const status = await Pure64.get_device_status();

// Calibrate keys
await Pure64.calibration_key([0, 1, 2, 3]);

// Get/set configuration
const config = await Pure64.get_key_config();
await Pure64.set_key_config(modifiedConfig);
await Pure64.save_key_config();
```

## Building WASM

### Requirements
- Rust toolchain
- wasm-pack: `cargo install wasm-pack`
- wasm32 target: `rustup target add wasm32-unknown-unknown`

### Build Commands

```bash
# Development build
cd src-wasm
cargo build --target wasm32-unknown-unknown

# Production build with wasm-pack
chmod +x scripts/build-wasm.sh
./scripts/build-wasm.sh
```

This will generate files in `wasm/`:
- `meowpad_wasm.js` - JavaScript bindings
- `meowpad_wasm_bg.wasm` - WebAssembly binary
- `meowpad_wasm.d.ts` - TypeScript definitions

## Environment Detection

The system automatically detects the runtime environment:

| Environment | Detection | Communication |
|-------------|-----------|---------------|
| **Tauri** | `window.__TAURI__` exists | Tauri IPC (invoke) |
| **Web** | `navigator.hid` exists | WebHID API via WASM |
| **Unknown** | Neither available | Error |

## WebHID Limitations

WebHID has some constraints compared to native USB:

1. **User Gesture Required**: Device selection must be triggered by user interaction
2. **HTTPS Only**: WebHID only works on HTTPS (or localhost for development)
3. **No Background**: Cannot communicate with device when tab is not active
4. **Permission Based**: User must explicitly grant permission for each device
5. **No Auto-Reconnect**: Must handle device disconnection/reconnection manually

## Current Implementation Status

✅ **Completed**
- Environment detection
- WASM module loader structure
- Pure64 WASM API wrapper
- Pure64 unified API (Tauri + WASM)
- Build scripts

⏳ **TODO**
- Load actual WASM module in loader.ts (currently placeholder)
- Test with real WebHID devices
- Implement other device types (MeowpadV2, V3, etc.)
- Handle device reconnection
- Error handling improvements
- Add device event listeners

## Migration Guide

To migrate existing code to use the unified API:

### Before (Tauri only)
```typescript
import * as api from '@/apis/pure64/api';

await api.connect();
const status = await api.get_device_status();
```

### After (Tauri + Web)
```typescript
import * as Pure64 from '@/wasm/pure64';

await Pure64.initialize();
await Pure64.connect();
const status = await Pure64.get_device_status();
```

The unified API maintains the same function signatures, so only import statements need to change.

## Troubleshooting

### WASM module not loading
- Check browser console for errors
- Ensure WebHID is available: `navigator.hid` should exist
- Verify WASM files are in `wasm/` directory
- Check browser compatibility (Chrome 89+, Firefox 98+)

### Device not connecting
- Click connect button (user gesture required)
- Check device permissions in browser settings
- Verify device is not already open by another app
- Look for WebHID permission prompt

### Build errors
- Install wasm-pack: `cargo install wasm-pack`
- Add wasm32 target: `rustup target add wasm32-unknown-unknown`
- Check Rust version (1.70+)

## Browser Compatibility

| Browser | WebHID Support | Status |
|---------|---------------|--------|
| Chrome/Edge | 89+ | ✅ Full support |
| Firefox | 98+ (flag) | ⚠️ Requires flag |
| Safari | 16.4+ | ⚠️ Limited |
| Opera | 75+ | ✅ Full support |

## License

Same as parent project.
