# NXP i.MX RT595S Peripheral Access Crate

This is a peripheral access crate (PAC) generated using svd2rust from the
official SVD file from the NXP-provided SDK.

To successfully build the SVD file was slightly modified:

- `INPUTMUX` `CT32BIT_CAP_SEL[%s]` had its address offset on the nested
  elements rather than on the container, causing miscalculation of the offsets
  of nested elements. This is moved to the parent.
- `CT32BIT_CAP_SEL[%s]` had `CT32BIT_CAP_SEL[%s]` (the same name) nested inside
  it, causing a conflicting definition. The nested one is renamed as
  `CT32BIT_CAP_SEL_SUB[%s]`.
