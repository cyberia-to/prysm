---
tags: prysm, cyb, chroma
alias: sigma assets, sigma neurons
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

resource chrome — mid-right

Sigma projects the robot's assets and manages its attached neurons, as defined by [anatomy](../../../cyb/anatomy.md). The Σ widget opens the assets/identities view. Avatar is the robot's visualization; Vault holds secrets and performs scoped signing.

## core function

sigma aggregates qualified balances across permitted attachments without merging subjects or foreign denominations. Every balance retains subject, network, asset identifier, source evidence, freshness and valuation basis. Observation, control and delegation are displayed separately; a total is not spendable authority. Identity selection is explicit in Sigma and cannot change an already captured action.

## Proposed host events

These are adapter intents/projections, not automatic cyberlinks. The [composition contract](../../system/specs/composition.md#navigation-and-authority) governs dispatch.

| receives from | token | meaning |
|---------------|-------|---------|
| host attachments | selection | explicitly selected subject/network changed — reload scoped projection |
| com | send | request transfer with captured subject, binding revision, network, asset and payload |

| sends to | token | meaning |
|----------|-------|---------|
| avatar view | resource | optional balance summary for display, without key access |
| spacetime | switch-renderer | open sigma assets/neurons in space zone |
| log/time | record | display persisted transfer attempts and outcomes from Cybergraph/BBG |

## widget layout (chrome slot)

```
glass [fix × fill(mid-right edge), depth overlay]
  stack vertical [align center]
    vector [Σ icon]
    counter [micro, total value]
    pill [emotion — green rising, red falling]
```

## Space view (assets and neurons)

opened when Σ widget tapped — renders in space zone:

```
glass [fill × fill, depth background, overflow scroll]
  stack vertical [gap 2g, padding 3g]
    --- header ---
    stack horizontal [gap 2g]
      vector [6g, Σ sigma icon]
      text [h2, "Sigma"]
      counter [h2, total portfolio value, right-aligned]
    --- qualified attachment row ---
    glass [fill × fix(6g), depth midground]
      stack horizontal [gap g]
        glass [fix(4g) × fix(4g), corner-radius 2g, green tint] — avatar circle
        address [big, with identity domain, network, access mode and hash bars]
        counter [body, total value, right-aligned]
    --- token list ---
    stack vertical [gap 0]
      glass [fill × auto, depth midground] — per token
        stack horizontal [gap g]
          vector [2g, token icon, colored]
          text [body, ticker "BOOT"]
          text [caption, ">"] — expandable
          pill [progress, green, proportion of portfolio]
          counter [body, balance "18 800"]
          counter [caption, price "6 .634"]
          counter [body, value "124 736"]
      saber [horizontal, g/8]
      ...
```

## tokens

each row: icon | ticker | balance | price | value. pill = share of total portfolio. expandable (>) for sub-tokens or LP details.

inputs: [[CYB]], [[HYDROGEN]], [[BOOT]], [[VOLT]], [[AMPERE]], IBC tokens, staking state, portfolio value
outputs: send/stake → authorized host action intent | navigate → token detail or the [Cyberver proposal](../../../aos/cyberver.md). Foreign staking/IBC fields follow their adapter profile; protocol resources are distinct from Body's hardware telemetry.

## Pages (proposed view)

- neurons: inspect, attach in observation/control/delegated mode, explicitly create, or detach; retain domain, network and device/key references
- [[coins]]: fungible token balances and transfers
- [[cards]]: unique tokens and collectibles
- [[scores]]: reputation and contribution metrics
- [[badges]]: achievement tokens

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 40g$): full table — icon + ticker + bar + balance + price + value
- $l_2$ ($w_{min} = 20g$): icon + ticker + balance + value (hide bar + price)
- $l_3$ ($w_{min} = 10g$, mobile): icon + ticker + value

## emotion

| element | emotion | trigger |
|---------|---------|---------|
| total portfolio counter | neutral unless a qualified change is known | show missing/stale valuation explicitly |
| token value | green if increased, red if decreased | balance change |
| pill progress | green | proportion |
| Σ widget glow | green increase, red decrease | balance change |

## states

| state | visual | trigger |
|-------|--------|---------|
| viewing | token list | default |
| sending | amount input + recipient in commander | tap send |
| receiving | QR code / address display | tap receive |
| loading | skeleton rows | fetching balances |
| observation only | balances visible, send unavailable | no controlled attachment/current grant |

## ECS

- Entity: sigma view
- Components:
  - `Sizing { width: Fill, height: Fill }`
  - `FoldSet { conformations }`
  - `TokenBalances { list of (icon, ticker, balance, price, value, proportion) }`
  - `TotalPortfolio { value }`
  - `SubjectDisplay { subject_ref, network, display_profile, access_mode }`
  - `AttachmentDisplay { binding_revision, vault_ref, grant_status }` — references only, never key bytes
- Systems:
  - `SigmaFetchSystem` reads qualified balance projections through State/host adapters
  - `SigmaSendSystem` emits exact transfer intent to the host's Ward/Vault path
  - `SigmaPortfolioSystem` computes proportions and total
