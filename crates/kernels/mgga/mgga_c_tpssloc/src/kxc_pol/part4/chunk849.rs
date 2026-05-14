//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 849/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk849<F: Float>(t13025: F, t2576: F, t225: F, t4266: F, t4143: F, t4145: F, t1496: F, t9541: F, t2427: F, t4101: F, t2528: F, t4199: F, t2663: F, t4211: F, t2535: F, t1471: F, t32: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13027 = 0.16666666666666666666e-2 * t2576 * t13025;
    let t13042 = t4266 * t225;
    let t13053 = t4143 * t225;
    let t13065 = t4145 * t225;
    let t13087 = t9541 * t1496;
    let t13105 = 8.0 * t2427 * t4101;
    let t13107 = t4199 * t2528;
    let t13109 = t4211 * t2663;
    let t13113 = t4199 * t2535;
    let t13115 = t32 * t1471;
    (t13027, t13042, t13053, t13065, t13087, t13105, t13107, t13109, t13113, t13115)
}
