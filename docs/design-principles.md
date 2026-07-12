# Design Principles

## Zero-cost abstraction

No dynamic dispatch.

## Async first

Every backend is asynchronous.

## Backend independence

Applications should not depend on backend-specific APIs.

## One operation = one type

Get

Put

Delete

are independent types instead of enum variants.

## Explicit over magic

No macros.

No code generation.

No reflection.