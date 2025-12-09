//! Custom buttons category: button assignments and customization.

use super::{Category, PropertyDef, PropertyValueType};
use crsdk_sys::DevicePropertyCode;

use DevicePropertyCode as C;
use PropertyValueType as V;

pub struct CustomButtons;

impl Category for CustomButtons {
    const PROPERTIES: &'static [PropertyDef] = &[
        PropertyDef::new(
            C::ButtonAssignmentAssignable1,
            "Assign C1",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable2,
            "Assign C2",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable3,
            "Assign C3",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable4,
            "Assign C4",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable5,
            "Assign C5",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable6,
            "Assign C6",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable7,
            "Assign C7",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable8,
            "Assign C8",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable9,
            "Assign C9",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable10,
            "Assign C10",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::ButtonAssignmentAssignable11,
            "Assign C11",
            "Function assigned to this custom button. Choose from commonly used functions for quick access.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton1,
            "Btn C1",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton2,
            "Btn C2",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton3,
            "Btn C3",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton4,
            "Btn C4",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton5,
            "Btn C5",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton6,
            "Btn C6",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton7,
            "Btn C7",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton8,
            "Btn C8",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton9,
            "Btn C9",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton10,
            "Btn C10",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButton11,
            "Btn C11",
            "Custom button on the camera body that can be assigned to frequently used functions.",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator1,
            "Btn Ind 1",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator2,
            "Btn Ind 2",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator3,
            "Btn Ind 3",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator4,
            "Btn Ind 4",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator5,
            "Btn Ind 5",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator6,
            "Btn Ind 6",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator7,
            "Btn Ind 7",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator8,
            "Btn Ind 8",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator9,
            "Btn Ind 9",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator10,
            "Btn Ind 10",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
        PropertyDef::new(
            C::AssignableButtonIndicator11,
            "Btn Ind 11",
            "Indicator state for this assignable button (active/inactive).",
            Some(V::Integer),
        ),
    ];
}
