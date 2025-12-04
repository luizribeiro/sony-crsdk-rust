use std::collections::HashMap;
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();

    let sdk_path = workspace_root.join("../app/CRSDK");
    let libs_path = workspace_root.join("libs");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=src/callback_shim.cpp");
    println!("cargo:rerun-if-changed={}", sdk_path.display());

    // Compile the C++ callback shim
    cc::Build::new()
        .cpp(true)
        .file("src/callback_shim.cpp")
        .include(&sdk_path)
        .flag("-std=c++17")
        .compile("callback_shim");

    // Link to Sony SDK libraries (copied to libs/ directory)
    let crsdk_lib = libs_path.join("crsdk");
    let adapters_lib = libs_path.join("adapters");
    let opencv_lib = libs_path.join("opencv");

    println!("cargo:rustc-link-search=native={}", crsdk_lib.display());
    println!("cargo:rustc-link-search=native={}", adapters_lib.display());
    println!("cargo:rustc-link-search=native={}", opencv_lib.display());

    // Link libraries
    println!("cargo:rustc-link-lib=dylib=Cr_Core");
    println!("cargo:rustc-link-lib=dylib=monitor_protocol");
    println!("cargo:rustc-link-lib=dylib=monitor_protocol_pf");
    println!("cargo:rustc-link-lib=dylib=Cr_PTP_IP");
    println!("cargo:rustc-link-lib=dylib=Cr_PTP_USB");
    println!("cargo:rustc-link-lib=dylib=ssh2");
    println!("cargo:rustc-link-lib=dylib=usb-1.0.0");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", sdk_path.display()))
        // Allow C++ features
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        // Allowlist SDK types and functions
        .allowlist_function("SCRSDK::.*")
        .allowlist_type("SCRSDK::.*")
        .allowlist_var("SCRSDK::.*")
        // Enable C++ namespace handling
        .enable_cxx_namespaces()
        // Generate comments from headers
        .generate_comments(true)
        // Use core instead of std
        .use_core()
        // Derive traits
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        // Handle opaque types
        .opaque_type("std::.*")
        .opaque_type(".*::basic_string.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    // Generate property code enum from bindings
    generate_property_codes(&bindings_path, &out_path);
}

fn generate_property_codes(bindings_path: &PathBuf, out_path: &PathBuf) {
    let content = fs::read_to_string(bindings_path).expect("Failed to read bindings.rs");

    // Parse property codes from bindings
    // Format may span multiple lines:
    // pub const CrDevicePropertyCode_CrDeviceProperty_FNumber:
    //     root::SCRSDK::CrDevicePropertyCode = 256;
    let mut properties: Vec<(String, String, u32)> = Vec::new();

    // Normalize: join lines and find all property code declarations
    let normalized = content.replace('\n', " ").replace("  ", " ");

    for cap in regex_find_property_codes(&normalized) {
        properties.push(cap);
    }

    // Sort by value for consistent ordering
    properties.sort_by_key(|(_, _, val)| *val);

    // Deduplicate by value - some SDK constants are aliases with the same value
    let mut seen_values = std::collections::HashSet::new();
    properties.retain(|(_, _, val)| seen_values.insert(*val));

    // Generate the enum
    let code = generate_enum_code(&properties);

    fs::write(out_path.join("property_codes.rs"), code).expect("Failed to write property_codes.rs");
}

fn regex_find_property_codes(content: &str) -> Vec<(String, String, u32)> {
    let mut results = Vec::new();

    // Find all occurrences of property code declarations
    let pattern = "pub const CrDevicePropertyCode_CrDeviceProperty_";
    let mut search_start = 0;

    while let Some(start) = content[search_start..].find(pattern) {
        let abs_start = search_start + start;
        let after_pattern = abs_start + pattern.len();

        // Find the colon (end of const name)
        if let Some(colon_offset) = content[after_pattern..].find(':') {
            let prop_name = &content[after_pattern..after_pattern + colon_offset];

            // Find the = sign and the value
            let after_colon = after_pattern + colon_offset;
            if let Some(eq_offset) = content[after_colon..].find('=') {
                let after_eq = after_colon + eq_offset + 1;

                // Find the semicolon
                if let Some(semi_offset) = content[after_eq..].find(';') {
                    let value_str = content[after_eq..after_eq + semi_offset].trim();

                    // Parse the value
                    let value = if value_str.starts_with("0x") || value_str.starts_with("0X") {
                        u32::from_str_radix(&value_str[2..], 16).ok()
                    } else {
                        value_str.parse().ok()
                    };

                    if let Some(val) = value {
                        let const_name =
                            format!("CrDevicePropertyCode_CrDeviceProperty_{}", prop_name);
                        results.push((const_name, prop_name.to_string(), val));
                    }
                }
            }
        }

        search_start = abs_start + 1;
    }

    results
}

fn to_pascal_case(s: &str) -> String {
    // The SDK names are already in PascalCase with underscores between words
    // e.g., "ExposureBiasCompensation", "Movie_Recording_Setting", "MediaSLOT1_Status"
    // We just need to remove underscores and preserve the existing casing
    let result: String = s
        .split('_')
        .map(|part| {
            if part.is_empty() {
                return String::new();
            }
            // Capitalize first letter of each part, preserve rest
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect();

    result
}

fn categorize_property(name: &str) -> &'static str {
    let lower = name.to_lowercase();

    if lower.contains("battery")
        || lower.contains("power")
        || lower.contains("temperature")
        || lower.contains("overheating")
    {
        "Power"
    } else if lower.contains("focus")
        || lower.contains("af_")
        || lower.starts_with("af")
        || lower.contains("nearfar")
    {
        "Focus"
    } else if lower.contains("movie")
        || lower.contains("recording")
        || lower.contains("proxy")
        || lower.contains("timecode")
        || lower.contains("framerate")
    {
        "Movie"
    } else if lower.contains("zoom") {
        "Zoom"
    } else if lower.contains("iso")
        || lower.contains("shutter")
        || lower.contains("fnumber")
        || lower.contains("aperture")
        || lower.contains("exposure")
        || lower.contains("gain")
        || lower.contains("iris")
    {
        "Exposure"
    } else if lower.contains("white")
        || lower.contains("colortemp")
        || lower.contains("wb")
        || lower.contains("awb")
    {
        "WhiteBalance"
    } else if lower.contains("media") || lower.contains("slot") {
        "Media"
    } else if lower.contains("flash") {
        "Flash"
    } else if lower.contains("image")
        || lower.contains("still")
        || lower.contains("file")
        || lower.contains("raw")
        || lower.contains("jpeg")
        || lower.contains("quality")
    {
        "Image"
    } else if lower.contains("lens") {
        "Lens"
    } else if lower.contains("audio") {
        "Audio"
    } else if lower.contains("picture")
        || lower.contains("creative")
        || lower.contains("gamma")
        || lower.contains("color")
    {
        "PictureProfile"
    } else if lower.contains("nd") || lower.contains("filter") {
        "NDFilter"
    } else if lower.contains("button") || lower.contains("dial") || lower.contains("assignable") {
        "CustomButtons"
    } else if lower.contains("silent") {
        "Silent"
    } else if lower.contains("stabilization") || lower.contains("steadyshot") {
        "Stabilization"
    } else if lower.contains("display")
        || lower.contains("monitor")
        || lower.contains("finder")
        || lower.contains("disp")
    {
        "Display"
    } else if lower.contains("drive")
        || lower.contains("bracket")
        || lower.contains("timer")
        || lower.contains("burst")
    {
        "Drive"
    } else if lower.contains("metering") {
        "Metering"
    } else {
        "Other"
    }
}

fn generate_enum_code(properties: &[(String, String, u32)]) -> String {
    let mut code = String::new();

    // Header
    writeln!(
        code,
        "// Auto-generated from Sony SDK headers - DO NOT EDIT"
    )
    .unwrap();
    writeln!(code, "// Generated by crsdk-sys build.rs").unwrap();
    writeln!(code).unwrap();

    // PropertyCategory enum
    writeln!(code, "/// Categories for camera properties").unwrap();
    writeln!(code, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(code, "#[non_exhaustive]").unwrap();
    writeln!(code, "pub enum PropertyCategory {{").unwrap();
    for cat in &[
        "Exposure",
        "Focus",
        "WhiteBalance",
        "Image",
        "Movie",
        "Media",
        "Power",
        "Zoom",
        "Lens",
        "Flash",
        "Audio",
        "PictureProfile",
        "NDFilter",
        "CustomButtons",
        "Silent",
        "Stabilization",
        "Display",
        "Drive",
        "Metering",
        "Other",
    ] {
        writeln!(code, "    {},", cat).unwrap();
    }
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    // Implement Display for PropertyCategory
    writeln!(code, "impl core::fmt::Display for PropertyCategory {{").unwrap();
    writeln!(
        code,
        "    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
    )
    .unwrap();
    writeln!(code, "        match self {{").unwrap();
    for cat in &[
        "Exposure",
        "Focus",
        "WhiteBalance",
        "Image",
        "Movie",
        "Media",
        "Power",
        "Zoom",
        "Lens",
        "Flash",
        "Audio",
        "PictureProfile",
        "NDFilter",
        "CustomButtons",
        "Silent",
        "Stabilization",
        "Display",
        "Drive",
        "Metering",
        "Other",
    ] {
        let display = match *cat {
            "WhiteBalance" => "White Balance",
            "PictureProfile" => "Picture Profile",
            "NDFilter" => "ND Filter",
            "CustomButtons" => "Custom Buttons",
            _ => cat,
        };
        writeln!(
            code,
            "            Self::{} => write!(f, \"{}\"),",
            cat, display
        )
        .unwrap();
    }
    writeln!(code, "        }}").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    // DevicePropertyCode enum
    writeln!(code, "/// Property codes for camera settings").unwrap();
    writeln!(code, "///").unwrap();
    writeln!(
        code,
        "/// This enum covers all {} properties exposed by the Sony Camera Remote SDK.",
        properties.len()
    )
    .unwrap();
    writeln!(code, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(code, "#[repr(u32)]").unwrap();
    writeln!(code, "#[non_exhaustive]").unwrap();
    writeln!(code, "pub enum DevicePropertyCode {{").unwrap();

    // Group by category for documentation
    let mut by_category: HashMap<&str, Vec<&(String, String, u32)>> = HashMap::new();
    for prop in properties {
        let cat = categorize_property(&prop.1);
        by_category.entry(cat).or_default().push(prop);
    }

    // Write variants
    for (const_name, prop_name, value) in properties {
        let variant_name = to_pascal_case(prop_name);
        // Skip reserved entries for cleaner API
        if prop_name.to_lowercase().contains("reserved") {
            continue;
        }
        writeln!(
            code,
            "    {} = crate::SCRSDK::{} as u32,",
            variant_name, const_name
        )
        .unwrap();
    }

    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    // Collect non-reserved properties for impl
    let non_reserved: Vec<_> = properties
        .iter()
        .filter(|(_, name, _)| !name.to_lowercase().contains("reserved"))
        .collect();

    // Implement methods
    writeln!(code, "impl DevicePropertyCode {{").unwrap();

    // ALL constant
    writeln!(code, "    /// All property codes").unwrap();
    writeln!(code, "    pub const ALL: &'static [Self] = &[").unwrap();
    for (_, prop_name, _) in &non_reserved {
        let variant_name = to_pascal_case(prop_name);
        writeln!(code, "        Self::{},", variant_name).unwrap();
    }
    writeln!(code, "    ];").unwrap();
    writeln!(code).unwrap();

    // as_raw
    writeln!(code, "    /// Get the raw SDK property code value").unwrap();
    writeln!(code, "    #[inline]").unwrap();
    writeln!(code, "    pub const fn as_raw(self) -> u32 {{").unwrap();
    writeln!(code, "        self as u32").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code).unwrap();

    // from_raw
    writeln!(code, "    /// Create from raw SDK property code").unwrap();
    writeln!(code, "    pub fn from_raw(code: u32) -> Option<Self> {{").unwrap();
    writeln!(code, "        match code {{").unwrap();
    for (const_name, prop_name, _) in &non_reserved {
        let variant_name = to_pascal_case(prop_name);
        writeln!(
            code,
            "            x if x == crate::SCRSDK::{} as u32 => Some(Self::{}),",
            const_name, variant_name
        )
        .unwrap();
    }
    writeln!(code, "            _ => None,").unwrap();
    writeln!(code, "        }}").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code).unwrap();

    // name
    writeln!(code, "    /// Get a human-readable name for this property").unwrap();
    writeln!(code, "    pub const fn name(self) -> &'static str {{").unwrap();
    writeln!(code, "        match self {{").unwrap();
    for (_, prop_name, _) in &non_reserved {
        let variant_name = to_pascal_case(prop_name);
        let display_name = humanize_property_name(prop_name);
        writeln!(
            code,
            "            Self::{} => \"{}\",",
            variant_name, display_name
        )
        .unwrap();
    }
    writeln!(code, "        }}").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code).unwrap();

    // category
    writeln!(code, "    /// Get the category for this property").unwrap();
    writeln!(
        code,
        "    pub const fn category(self) -> PropertyCategory {{"
    )
    .unwrap();
    writeln!(code, "        match self {{").unwrap();
    for (_, prop_name, _) in &non_reserved {
        let variant_name = to_pascal_case(prop_name);
        let cat = categorize_property(prop_name);
        writeln!(
            code,
            "            Self::{} => PropertyCategory::{},",
            variant_name, cat
        )
        .unwrap();
    }
    writeln!(code, "        }}").unwrap();
    writeln!(code, "    }}").unwrap();

    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    // Display impl
    writeln!(code, "impl core::fmt::Display for DevicePropertyCode {{").unwrap();
    writeln!(
        code,
        "    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
    )
    .unwrap();
    writeln!(code, "        write!(f, \"{{}}\", self.name())").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}").unwrap();

    code
}

fn humanize_property_name(name: &str) -> String {
    // Convert SDK property name to human-readable format
    // e.g., "Movie_Recording_Setting" -> "Movie Recording Setting"
    // e.g., "FNumber" -> "F-Number"
    // e.g., "IsoSensitivity" -> "ISO Sensitivity"

    let mut result = String::new();
    let mut prev_was_upper = false;
    let mut prev_was_underscore = true;

    for c in name.chars() {
        if c == '_' {
            result.push(' ');
            prev_was_underscore = true;
            prev_was_upper = false;
        } else if c.is_uppercase() {
            if !prev_was_upper && !prev_was_underscore && !result.is_empty() {
                result.push(' ');
            }
            result.push(c);
            prev_was_upper = true;
            prev_was_underscore = false;
        } else {
            result.push(c);
            prev_was_upper = false;
            prev_was_underscore = false;
        }
    }

    // Fix common abbreviations
    result
        .replace("F Number", "F-Number")
        .replace("Iso ", "ISO ")
        .replace(" Iso", " ISO")
        .replace("Af ", "AF ")
        .replace(" Af", " AF")
        .replace("Wb ", "WB ")
        .replace(" Wb", " WB")
        .replace("Awb", "AWB")
        .replace("Dro", "DRO")
        .replace("Nd ", "ND ")
        .replace(" Nd", " ND")
        .replace("Hdmi", "HDMI")
        .replace("Usb", "USB")
        .replace("Ftp", "FTP")
        .replace("Lut", "LUT")
        .replace("Sq ", "S&Q ")
        .replace("Ecs", "ECS")
        .replace("Ptz", "PTZ")
        .replace(" Id", " ID")
}
