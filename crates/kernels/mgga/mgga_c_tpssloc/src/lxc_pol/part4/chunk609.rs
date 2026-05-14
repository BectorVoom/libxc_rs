//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 609/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk609<F: Float>(t40: F, t52: F, t4072: F, t510: F, t1774: F, t671: F, t1409: F, t2433: F, t3966: F, t607: F, t73: F, t2440: F, t76: F, t157: F, t182: F, t145: F, t185: F, t1472: F, t751: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t4073 = t510 * t4072;
    let t4077 = t1774 * t671;
    let t4080 = t2433 * t1409;
    let t4086 = piecewise3(t146, 0.0, 4.0 / 9.0 * t4080 * t607 + 4.0 / 3.0 * t73 * t3966);
    let t4087 = t2440 * t1409;
    let t4093 = piecewise3(t150, 0.0, 4.0 / 9.0 * t4087 * t607 - 4.0 / 3.0 * t76 * t3966);
    let t4094 = t4086 + t4093;
    let t4095 = t4094 * t157;
    let t4097 = 0.19751673498613801407e-1 * t4095 * t182;
    let t4098 = t145 * t4094;
    let t4099 = t4098 * t185;
    let t4100 = t1472 * t751;
    (t4073, t4077, t4080, t4087, t4094, t4095, t4097, t4098, t4099, t4100)
}
