//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1287/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1287<F: Float>(t1333: F, t61870: F, t19590: F, t61873: F, t18394: F, t3532: F, t18546: F, t6242: F, t4466: F, t60738: F, t4425: F, t60707: F) -> (F, F, F, F, F, F, F) {
    let t65440 = t61870 * t1333;
    let t65442 = t61873 * t19590;
    let t65444 = t18394 * t3532;
    let t65533 = t6242 * t18546;
    let t65551 = t60738 * t4466;
    let t65561 = t60738 * t4425;
    let t65564 = F::new(119.0) / F::new(3456.0) * t60707;
    (t65440, t65442, t65444, t65533, t65551, t65561, t65564)
}
