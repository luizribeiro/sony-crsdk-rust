# Property Type Analyzer

Analyze whether a camera property is using the correct value type, or if it should use a more specific typed value instead of `V::Integer`.

## Property to Analyze
$ARGUMENTS

## Analysis Steps

### 1. Find the Property Definition
Search in `crsdk/src/property/categories/*.rs` to find how this property is currently defined (display name, description, value type).

### 2. Check Camera Output
Run `cargo run --release --bin sonyctl -- props list 2>/dev/null | grep -i "<property_name>"` to see:
- The actual value returned by the camera
- Available values (discrete list or range)
- Whether the raw value looks like it needs formatting

### 3. Check SDK Headers
Search in `../app/CRSDK/CrDeviceProperty.h` for the property code constant to find:
- Any enum definitions for this property's values
- Comments explaining what values mean
- Related constants (e.g., `CrXxx_On = 1, CrXxx_Off = 2`)

### 4. Check Existing Typed Values
Review `crsdk/src/property/values/*.rs` to see if:
- An existing typed value already fits this property
- The property values match patterns like On/Off, Auto/Manual, Switch, etc.

### 5. Determine Recommendation

Output one of these verdicts:

**CORRECT** - V::Integer is appropriate because:
- Values are truly arbitrary integers (positions, counts, IDs)
- Values are ranges with no semantic meaning beyond the number itself

**CHANGE TO EXISTING** - Should use existing type `V::XxxYyy` because:
- Values match an existing enum pattern (OnOff, Switch, AutoManual, etc.)
- The SDK defines matching constants

**NEEDS NEW TYPE** - Requires a new typed value because:
- SDK defines specific enum values that don't match existing types
- Values have semantic meaning that should be formatted (like MovieQuality)

## Output Format

```
Property: <PropertyCode>
Current: V::Integer
Verdict: CORRECT | CHANGE TO EXISTING | NEEDS NEW TYPE
Reason: <explanation>
Action: <none | change to V::XxxYyy | create new type with values: ...>
```
