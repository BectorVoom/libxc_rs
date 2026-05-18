//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1208/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1208<F: Float>(t28395: F, t815: F, t23097: F, t1516: F, t25068: F, t5624: F, t6621: F, t5572: F, t6581: F, t23141: F, t23144: F, t25109: F, t25126: F, t25133: F, t26644: F, t26646: F, t28380: F, t28384: F, t28386: F, t28390: F) -> (F, F) {
    let t28396 = t815 * t28395;
    let t28397 = t23097 * t28396;
    let t28399 = t25068 * t1516;
    let t28401 = t6621 * t5624;
    let t28403 = t6581 * t5572;
    let t28405 = F::new(0.16956557559538964159e-1) * t25109 + t28380 / F::new(192.0) - F::new(0.12111826828242117256e-2) * t28384 + t28386 / F::new(16.0) + F::new(0.84782787797694820792e-2) * t28390 + F::new(0.28260929265898273598e-2) * t25126 + F::new(0.6728792682356731809e-4) * t25133 + F::new(0.24223653656484234512e-2) * t28397 + t26644 - t28399 / F::new(192.0) + F::new(5.0) / F::new(384.0) * t28401 + t26646 - t28403 / F::new(48.0) + t23141 + t23144;
    (t28396, t28405)
}
