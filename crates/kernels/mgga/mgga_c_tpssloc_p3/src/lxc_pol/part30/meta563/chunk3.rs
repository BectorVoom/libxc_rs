//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1928/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1928<F: Float>(t28395: F, t815: F, t23097: F, t1516: F, t25068: F, t5624: F, t6621: F, t5572: F, t6581: F, t23141: F, t23144: F, t25109: F, t25126: F, t25133: F, t26644: F, t26646: F, t28380: F, t28384: F, t28386: F, t28390: F) -> (F, F) {
    let t28396 = t815 * t28395;
    let t28397 = t23097 * t28396;
    let t28399 = t25068 * t1516;
    let t28401 = t6621 * t5624;
    let t28403 = t6581 * t5572;
    let t28405 = F::cast_from(0.16956557559538964159e-1_f64) * t25109 + t28380 / F::cast_from(192.0_f64) - F::cast_from(0.12111826828242117256e-2_f64) * t28384 + t28386 / F::cast_from(16.0_f64) + F::cast_from(0.84782787797694820792e-2_f64) * t28390 + F::cast_from(0.28260929265898273598e-2_f64) * t25126 + F::cast_from(0.6728792682356731809e-4_f64) * t25133 + F::cast_from(0.24223653656484234512e-2_f64) * t28397 + t26644 - t28399 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t28401 + t26646 - t28403 / F::cast_from(48.0_f64) + t23141 + t23144;
    (t28396, t28405)
}
