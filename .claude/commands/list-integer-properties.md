# List Integer Properties Needing Analysis

Extract properties using V::Integer that likely need a more specific type.

## Steps

1. Run this to get all Integer-typed properties with their display names:
```bash
grep -B2 "Some(V::Integer)" crsdk/src/property/categories/*.rs | grep -E "C::|display name"
```

2. Filter OUT properties that are clearly numeric by name pattern:
   - *Position*, *Pos*, *Level*, *Index*, *Number*, *Count*, *Size*, *Time*, *Duration*
   - *Preset*, *Speed*, *Rate*, *Version*, *Status*, *State*
   - *ID*, *Num*, *Value*, *Limit*, *Min*, *Max*, *Step*

3. Flag properties that likely need typed values:
   - *Mode*, *Type*, *Format*, *Setting*, *Control*, *Target*
   - Properties where SDK likely defines enum constants

4. Output a prioritized list for parallel analysis.
