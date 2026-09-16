---
tags: prysm, cyb, chroma
alias: settings, configuration
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

settings view — accessible from the robot menu

Settings presents Soul configuration, Sigma attachment management, Vault key references and adapter configuration. Sidebar sections are UI groupings, not organs or signing subjects.

Not a chrome slot: opens as a view from the robot menu. Avatar visualizes the robot; menu placement grants no identity or key ownership.

## structure

```
glass [fill × fill, depth background]
  stack horizontal [gap 0]
    --- left: settings menu ---
    stack vertical [fix(20g) × auto, gap 2g]
      ion + text [Keys]
      ion + text [Drive]
      ion + text [Signer]
      ion + text [Tokens]
      ion + text [Networks]
      ion + text [Channels]
      ion + text [Hotkeys]
      ion + text [LLM]
    --- right: active section content ---
    glass [fill × fill, depth midground, overflow scroll]
```

## sections

### Keys

Manage qualified neuron attachments and Vault references. Show observation/control/delegated mode, domain, network and binding revision. Key creation, import, rotation and attachment are distinct explicit actions; display-address reuse cannot merge subjects.

```
glass [fill × auto, depth midground]
  raster [hardware key image]
  pill [green, "active"]
  text [caption, "key"] + pill [neutral, "cybergirl"]
  text [caption, "at path"] + pill [neutral, "44/118/0/0/0"]
  text [caption, "from neuron"] + address [small, with hash bars]
  text [caption, "attachment"] + pill [neutral, "control / delegate / observe"] + text [caption, "subject and network:"]
```

Commander offers explicit attach, observe, create or import flows. The derivation path above is a foreign-profile example, not a universal identity derivation. Secret key material never enters view props or logs.

### Drive

Storage/content adapter status and configuration. The following is a historical web-backend display sketch; current status must come from the actual host components (Cybergraph/BBG, Radio, Soma and supported content adapters). It does not mandate a second database or legacy ML engine.

```
text [h3, "Backend status"]
  db started (queries: 0)
  ipfs started (total: 0 | db - 0 node - 0 gateway - 0)
  rune started
  ml started — featureExtractor: ready
  sync started — transactions: listen, log cyberlinks: active, particles: active
button [green saber frame, "download logs"]
input [text, "enter sentence..."] + button ["Search by embedding"]
--- backend config ---
text [caption, "api"] + input [text, URL] + button ["edit"]
text [caption, "gateway"] + input [text, URL] + button ["edit"]
button ["Reconnect"]
```

### Signer

```
text [h2, "Action review preferences"]
toggle ["show optional review when policy permits"]
```

Changing prompt preferences cannot disable Ward checks, current revocation, payload binding or Vault scope. Mandatory review remains mandatory under the active policy.

### Tokens

```
table [sortable]
  columns: id ▲ | contract ▲ | channel id ▲ | ticker ▲ | logo ▲ | decimals ▲ | chain id ▲
```

### Networks

```
table [sortable]
  columns: id ▲ | chainId ▲ | name ▲ | logo ▲ | genesis hash ▲ | prefix ▲
```

### Channels

```
table [sortable]
  columns: id ▲ | active ▲ (green dot) | destination chain id ▲ | destination channel id ▲ | source chain id ▲ | source channel id ▲
```

### Hotkeys

```
table
  columns: Hotkey | Page | Description
  rows:
    / | all | Focus commander
    tab, enter | all | action bar keyboard navigation
    f | graph | Toggle graph fullscreen
```

### LLM

```
text [h2, "LLM"]
text [body, "configured provider / model revision"]
pill [neutral, "Soma model profile"]
```

Model and provider choices belong to Soul/Soma configuration. Remote inference access, credential references and data disclosure require their declared policy; selecting a model does not grant signing authority. Foreign token/network/channel fields above retain adapter-defined bytes and meanings.

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 40g$): sidebar + content side by side
- $l_2$ ($w_{min} = 20g$): sidebar above content
- $l_3$ ($w_{min} = 10g$, mobile): sidebar collapsed to icons, content full width

## emotion

- sidebar active item: green text + colored icon
- Drive status: green = running, red = stopped
- Keys "active" pill: green
- Channel active dots: green

## ECS

- Entity: settings view
- Components:
  - `Sizing { width: Fill, height: Fill }`
  - `FoldSet { conformations }`
  - `ActiveSection { keys | drive | signer | tokens | networks | channels | hotkeys | llm }`
  - `BackendStatus { db, ipfs, rune, ml, sync }`
  - `TokenRegistry { list of token entries }`
  - `NetworkRegistry { list of network entries }`
  - `ChannelRegistry { list of channel entries }`
- Systems:
  - `SettingsMenuSystem` handles section navigation
  - `SettingsBackendSystem` manages backend connection, logs, reconnect
  - `SettingsKeySystem` emits scoped attachment/key-management intents through Sigma/Ward/Vault
  - `SettingsRegistrySystem` manages tokens/networks/channels tables
