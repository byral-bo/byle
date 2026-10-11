# ADR-0001: Layered Platform Architecture

**Status:** Approved 
**Date:** 2026-07-03

## Context

Byle needs an architectural model that supports:
- A single language for all platforms
- Platform-independent APIs
- Long-term evolution
- Independent subsystem development

## Decision

Adopt a strictly layered architecture where each layer only depends on the layer immediately below it.

```
Applications
 │
Byle UI
 │
Byle Scene
 │
Byle Graphics
 │
Byle Windowing
 │
Byle Platform
 │
Byle Runtime
 │
Byle Language (Compiler)
```

The Package Manager is horizontal — it servis all layers equally.

## Consequences

**Positive:**
- Any layer can be replaced independently
- Platform-specific code is isolated in adapters
- The compiler is decoupled from platform concerns
- Applications are portable across all supported platforms

**Negative:**
- More layers mean more abstraction overhead
- Requiris discipline to maintain layer boundaries
