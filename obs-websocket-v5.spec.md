# OBS WebSocket v5 Protocol Spec

Full current spec at: https://raw.githubusercontent.com/obsproject/obs-websocket/master/docs/generated/protocol.md 

## Connection Flow
1. Connect WebSocket
2. Receive `Hello` (OpCode 0) with auth challenge if required
3. Send `Identify` (OpCode 1) with auth string and RPC version
4. Receive `Identified` (OpCode 2) - ready for requests

## Authentication
SHA256-based: `base64(SHA256(password + salt) + challenge)`

## Message Structure
```json
{
  "op": number,    // OpCode
  "d": object      // Data payload
}
```

## Key OpCodes
- `0`: Hello - Server init
- `1`: Identify - Client auth
- `2`: Identified - Auth success
- `5`: Event - Notifications
- `6`: Request - Client commands
- `7`: RequestResponse - Server replies

## Request/Response Pattern
```json
// Request
{
  "op": 6,
  "d": {
    "requestType": "GetSceneList",
    "requestId": "uuid",
    "requestData": {}
  }
}

// Response
{
  "op": 7,
  "d": {
    "requestType": "GetSceneList",
    "requestId": "uuid",
    "requestStatus": {"result": true, "code": 100},
    "responseData": {}
  }
}
```

## Core Request Categories

### General
- `GetVersion` - Protocol/version info
- `GetStats` - Performance stats
- `GetHotkeyList` - Available hotkeys
- `TriggerHotkeyByName` - Execute hotkey

### Scenes
- `GetSceneList` - All scenes
- `GetCurrentProgramScene` - Active scene
- `SetCurrentProgramScene` - Switch scene
- `CreateScene` - New scene
- `RemoveScene` - Delete scene

### Inputs (Sources)
- `GetInputList` - All inputs
- `CreateInput` - New source
- `GetInputSettings` - Source config
- `SetInputSettings` - Update source
- `SetInputMute` - Audio control

### Scene Items
- `GetSceneItemList` - Items in scene
- `CreateSceneItem` - Add source to scene
- `SetSceneItemTransform` - Position/scale
- `SetSceneItemEnabled` - Visibility

### Outputs
- `GetStreamStatus` - Stream state
- `StartStream`/`StopStream` - Stream control
- `GetRecordStatus` - Recording state
- `StartRecord`/`StopRecord` - Record control

### Filters
- `GetSourceFilterList` - Source filters
- `CreateSourceFilter` - Add filter
- `SetSourceFilterSettings` - Configure filter

## Event Subscriptions
Bitmask for event types:
- `General`: 1<<0
- `Scenes`: 1<<2  
- `Inputs`: 1<<3
- `Outputs`: 1<<6
- `All`: All non-high-volume events

## Error Codes
- `100`: Success
- `203`: Missing request type
- `204`: Unknown request type
- `400`: Invalid request field
- `600`: Resource not found

## Common Patterns
- Use UUIDs when available for stable references
- Name-based fallbacks supported
- Batch requests available (OpCode 8/9)
- Event-driven updates for state changes