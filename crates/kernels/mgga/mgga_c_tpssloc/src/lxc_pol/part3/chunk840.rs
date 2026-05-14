//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 840/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk840<F: Float>(t2833: F, t699: F, t241: F, t2978: F, t2955: F, t969: F, t2967: F, t964: F, t340: F, t63: F, t344: F, t221: F, t339: F, t2960: F, t2974: F, t135: F, t3016: F) -> (F, F, F, F, F, F, F, F) {
    let t10302 = t699 * t2833;
    let t10304 = t241 * t2978;
    let t10331 = t2955 * t969;
    let t10333 = t964 * t2967;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = 0.3086419753086419753e-3 * t339 * t10337;
    let t10342 = t2960 * t2974;
    let t10352 = t135 * t3016;
    (t10302, t10304, t10331, t10333, t10335, t10339, t10342, t10352)
}
