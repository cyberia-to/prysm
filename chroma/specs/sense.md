---
tags: prysm, cyb, chroma
alias: notify, senses, perception
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

notifications chrome — mid-left

**notify**: ambient signals from the world. incoming messages, system events, sensor data. the sense chrome is where the world enters — before language, before computation, raw contact between an agent and its environment

Also the full [Sense](../../../cyb/parts/sense.md) view: conversations with neurons and robots, particle history, and a proposed projection of Soma conversations. Soma owns cognition; Vision, Voice and Body own their sensory/device responsibilities.

## sense as concept

The interface presents perception and communication. Cameras, microphones, chemical sensors and eyes are sensory sources, not automatically protocol neurons. A neuron authors a link to their data under an explicit attribution profile; a device may serve several subjects, and a subject may use several devices. Generated and imported particles need not be claims of direct sensory observation. Cyb renders these sources without inventing authorship or authentication.

modalities — vision, hearing, touch, taste, smell, proprioception, thermoception, nociception, equilibrioception. the graph must handle all of them: images, sounds, chemical data, spatial coordinates

qualia — the subjective quality of experience. qualia resist reduction. they are why a superintelligence that only processes symbols is incomplete — it must also receive the world directly

## protocol role

molecule in the element tree $\mathcal{T}$. membrane = mid-left zone of [[prysm/grid]]. S widget on left edge shows unread count + emotion of latest signal.

## core function

the sense chrome slot shows an unread count and the emotional state of the latest incoming signal. any chroma can post a notification via `(*, sense, notify, …)`. sense never disrupts spacetime — it accumulates signals at the edge and lets the user decide when to open them.

## Proposed host events

Local notification events do not imply graph publication. Conversations preserve remote author, local acting attachment, identity domain, network and disclosure policy; a channel or model selection grants no authority.

| receives from | token | meaning |
|---------------|-------|---------|
| any | notify | new signal — increment unread, update emotion |
| spacetime | output | filter by explicit attachment/conversation; preserve independent authors |

| sends to | token | meaning |
|----------|-------|---------|
| spacetime | switch-renderer | open sense messenger in space zone |

## widget layout (chrome slot)

```
glass [fix × fill(mid-left edge), depth overlay]
  stack vertical [align center]
    vector [S icon]
    counter [micro, unread count, green]
    pill [emotion of latest signal]
```

S widget glow reflects [[emotion]] of latest incoming message

## Space view (messenger)

opened when S widget tapped — renders in space zone:

```
glass [fill × fill, depth background]
  stack horizontal [gap 0]
    --- left: conversation list ---
    glass [fix(25g) × fill, depth midground, overflow scroll]
      tabs [All | # | @ | llm]
      stack vertical [gap 0]
        glass [fill × fix(8g)] — per conversation
          stack horizontal [gap g]
            glass [fix(5g) × fix(5g), corner-radius 5g/2] — avatar
            stack vertical
              text [body, "@mastercyb"]
              text [caption, "last message preview"]
            text [micro, "24/03"]
            counter [micro, message count badge]
    --- right: active conversation ---
    glass [fill × fill, depth midground, overflow scroll]
```

### tabs

**All** — all conversations combined, sorted by last activity

**# (particle history)** — interactions with a particle, with retained author and source evidence

**@ (neuron chats)** — direct messaging. chat bubbles: incoming left, outgoing right. a supported messaging profile may publish a [[cyberlink]] with [[conviction]]. Local drafts, transport messages and final network records remain distinct

**llm** — a view of Soma tasks and conversations, configured through Soul. Model outputs are proposals until the host authorizes an action. Commander offers ask, attach and edit intents; model/provider selection is separate from the acting neuron.

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 40g$): conversation list + active chat side by side
- $l_2$ ($w_{min} = 20g$): tap conversation → full-width chat (push navigation)
- $l_3$ ($w_{min} = 10g$, mobile): same as $l_2$, compact

## emotion

| element | emotion | trigger |
|---------|---------|---------|
| unread badge | green | new messages |
| token amount on message | green | conviction attached |
| S widget glow | [[emotion]] of latest signal | incoming content |
| message status | labeled local / sent / accepted / final / failed / unknown | show delivery, remote acceptance and finality separately |
| llm microphone | red | recording active |

## states

| state | visual | trigger |
|-------|--------|---------|
| idle | S icon, count, emotion pill | — |
| list view | conversation list | widget tapped |
| chat view | active conversation | conversation tapped |
| composing | commander input focused | typing |
| recording | microphone active, red indicator | voice input |

## ECS

- Entity: sense view
- Components:
  - `Sizing { width: Fill, height: Fill }`
  - `FoldSet { conformations }`
  - `ActiveTab { all | particle_history | neuron_chat | llm }`
  - `Conversations { list of (id, subject_refs, network, display_image, name, last_message, timestamp, unread_count, evidence) }`
  - `ActiveConversation { conversation_id }`
  - `LlmModel { provider, model_name }`
- Systems:
  - `SenseConversationSystem` fetches conversation list from cybergraph
  - `SenseMessageSystem` reads message projections and emits captured send intent through host adapters
  - `SenseLlmSystem` presents Soma task/conversation state
  - `SenseNotificationSystem` updates S widget with unread count + emotion
