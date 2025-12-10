# Audit Property Types in Category

Analyze all V::Integer properties in a category file to find those needing better types.

## Category to Audit
$ARGUMENTS

## Analysis Process

### 1. Extract Integer Properties
From `crsdk/src/property/categories/$ARGUMENTS.rs`, list all properties using `Some(V::Integer)`.

### 2. Cross-Reference with SDK
For each property code, search `../app/CRSDK/CrDeviceProperty.h` for:
- Matching enum definition (e.g., `CrPropertyName` enum)
- Value constants with semantic meaning

### 3. Categorize Each Property

**Skip** (keep as Integer):
- Numeric ranges (min/max/step values)
- Positions, coordinates, percentages
- Counts, sizes, durations, IDs
- Version numbers, timestamps

**Flag for change**:
- Properties with SDK enum definitions
- Binary states (should be OnOff, Switch, AutoManual)
- Mode/Type/Format properties with discrete values
- Settings matching existing typed values

### 4. Check Existing Types
Review `crsdk/src/property/values/*.rs` for reusable types:
- OnOff, Switch, AutoManual, LockIndicator
- Percentage, Integer (legitimate uses)
- Domain-specific: MovieQuality, MovieFileFormat, etc.

## Output Format

```markdown
## Category: $ARGUMENTS

### Properties Correctly Using Integer
- PropertyCodeA: numeric range, no semantic meaning
- PropertyCodeB: position value

### Properties Needing Type Changes
| Property | Current | Recommended | SDK Enum | Reason |
|----------|---------|-------------|----------|--------|
| PropX | Integer | OnOff | CrPropX | Binary on=1/off=2 |
| PropY | Integer | NEW TYPE | CrPropY | Has 5 semantic values |

### New Types Needed
1. **TypeName**: PropX, PropY, PropZ share enum pattern
   - Value1 = 0x01: "Label1"
   - Value2 = 0x02: "Label2"
```
