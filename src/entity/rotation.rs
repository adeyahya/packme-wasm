use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rotation {
    // RotationLWH (l, w, h)
    LWH,
    // RotationWLH (w, l, h)
    WLH,
    // RotationWHL (w, h, l)
    WHL,
    // RotationHLW (h, l, w)
    HLW,
    // RotationHWL (h, w, l)
    HWL,
    // RotationLHW (l, h, w)
    LHW,
}
