# Media

Sigil treats photos and audio as sensitive data with their own normalization, encryption and retention rules.

## Images

The privacy-preserving default is not to forward the source file byte-for-byte.

```text
source image
    |
decoder
    |
pixel buffer
    |
controlled re-encode
    |
chunk planner
    |
independent media key
    |
authenticated encrypted chunks
```

Decoding to pixels and re-encoding creates a new file representation and lets Sigil discard source metadata such as:

- EXIF/XMP fields
- GPS coordinates
- original timestamps
- device model fields
- embedded thumbnails
- original filename/comment fields

The output encoder should use a small, controlled metadata policy.

This process does not remove identifying information visible in the pixels themselves.

## Voice messages

Voice recording should avoid creating a long-lived plaintext recording file when the platform allows streaming processing.

```text
microphone
    |
audio frames / PCM
    |
normalization
    |
Opus encoder
    |
chunk planner
    |
independent media key
    |
authenticated encrypted chunks
```

The goal is to encrypt progressively and keep plaintext audio buffers short-lived.

The audio path does not promise voice anonymity. A person's voice, room acoustics or background sounds may still be identifying.

## Chunking

Chunking is for delivery reliability and bounded memory usage. It is not encryption.

Each media object should have an encrypted manifest containing only the information the receiver needs, for example:

```text
media type
canonical dimensions or duration
chunk count
chunk ordering
content key/material reference
```

The relay should see ciphertext chunk sizes and opaque delivery identifiers, not plaintext media metadata.

## Received media

The normal receive path should keep media inside Sigil's private storage/rendering domain.

```text
encrypted chunks
    |
authenticated decrypt
    |
validated decoder
    |
render / playback
```

Media should not automatically appear in the system gallery, shared downloads directory or another application's media index.

Export is an explicit user action and leaves Sigil's protected storage boundary.

## Parser hardening

Image and audio decoders process attacker-controlled input and are therefore part of the attack surface.

Implementation should use:

- strict size and dimension limits
- bounded memory use
- format allowlists
- isolated decoder processes/sandboxes where practical
- canonical re-encoding rather than arbitrary metadata preservation
- fuzzing for custom parsing or framing code

## Original-file mode

A future explicit original-file transfer mode may be useful for documents or forensic-quality media, but it must be clearly different from the privacy-preserving default because it can preserve source metadata.
