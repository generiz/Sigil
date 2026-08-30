# Ephemeral visual layer

Sigil separates human recognition from renderer state and network identity.

## Stable to the human

A verified contact may be represented in the secure conversation surface by a minimal local marker: color, shape and pattern.

Example:

```text
      ●
   verified
```

The human can learn that marker as "the person I verified" without the interface needing to display a real name, phone number or global username.

The marker has no authentication authority. Authentication still comes from the pinned cryptographic identity key.

## Local derivation

The Rust core models a `ContactVisualMarker` derived from:

```text
verified public identity key
+
device-local visual secret
```

The resulting palette/shape/pattern slots are local presentation values. They are not transmitted as peer identifiers.

A server that sees network traffic should not need to know which local color or shape represents a peer on a device.

## Ephemeral renderer state

Stable human recognition does not require stable internal rendering tokens.

Each sensitive render period can use a fresh `VisualRenderEpoch` with a random epoch identifier and short-lived render token:

```text
local marker
    |
visual epoch 41 -> random render token
    |
visual epoch 42 -> different render token
    |
visual epoch 43 -> different render token
```

The eventual Android renderer may use those epochs to derive transient shader/uniform state while preserving the same human-visible marker.

The current repository models the state boundary only. It does not yet contain the Android GPU renderer.

## No OS text semantics

The intended sensitive receive view should avoid normal text widgets for message content and identity presentation where practical. Custom glyph geometry and a dedicated render surface remain the preferred direction.

This reduces framework-level semantic exposure. It does not make the display invisible to the operating system.

## Physical boundary

At the last stage the device must generate actual pixels. If the user sees green, the display pipeline necessarily produces the corresponding optical/pixel information.

A compromised kernel, GPU driver, compositor, framebuffer capture path or physical camera can still observe the rendered output.

Therefore the visual layer is endpoint hardening, not encryption and not an anti-screen-capture guarantee.

## Rotation boundaries

Sigil deliberately keeps these lifetimes independent:

- contact visual marker: locally stable enough for human recognition
- visual render epoch: short-lived
- random alias: rotatable presentation aid
- delivery epoch: short-lived network state
- message key: one ratchet step/message as defined by the future crypto protocol
- identity key: long-term verification anchor

This separation lets the user retain continuity while the network and renderer avoid unnecessary stable identifiers.
