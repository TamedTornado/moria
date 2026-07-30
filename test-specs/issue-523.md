# Issue 523 — Implement the independent commitment and replay oracle

References: `validation.md` TECH-059; `architecture.md` TECH-008/009; `content-persistence.md` TECH-047; issue M-082.

## Properties

- Independent reference encodes every canonical record/hash domain and reproduces configuration, world, replay prefix, and active-history digests without production encoder/hash-fold implementation reuse.
- For all one-brick changes, reference identifies exact changed leaf/ancestor work; for all derived-only changes, canonical root remains identical.

## Configurations

- Golden empty/populated domains; Genesis versus Confirmed(0); matter/placement/ID/allocator/domain/participant/RNG changes; participant events; ordinary replay streams; valid correction branches; checkpoint anchors.
- Independently fold physical records, retain superseded bytes, and derive one corrected semantic log plus both digest domains.
- Compare participant and RNG commitments in sorted IDs/stream order.

## Error paths

- Reject domain/tag/length/checksum mismatch, unsorted children, physical identity contamination, invalid branch target/root/history/count/gaps, and trailing bytes.
- Poison each canonical influence and identify first changed leaf/root/tick; poison cache/mesh/telemetry/adapter and require no digest change.
- Any production transition/hash helper dependency invalidates independence.
