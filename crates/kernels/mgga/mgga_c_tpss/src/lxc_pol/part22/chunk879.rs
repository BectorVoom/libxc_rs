//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 879/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk879<F: Float>(t8561: F, t9080: F, t2703: F, t2786: F, t2715: F, t8549: F, t8548: F, t2724: F, t940: F, t2813: F, t375: F, t1071: F, t2997: F, t3000: F, t433: F, t275: F, t400: F, t8662: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9081 = t9080 * t8561;
    let t9089 = t2786 * t2703;
    let t9093 = t8549 * t2715;
    let t9094 = t8548 * t9093;
    let t9095 = t9080 * t2724;
    let t9116 = t8549 * t940;
    let t9117 = t8548 * t9116;
    let t9133 = 1.0 / t2813 / t375;
    let t9172 = 1.0 / t2997 / t1071;
    let t9176 = 1.0 / t3000 / t433;
    let t9181 = t275 * t8662 * t400;
    (t9081, t9089, t9094, t9095, t9117, t9133, t9172, t9176, t9181)
}
