//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 877/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk877<F: Float>(t10325: F, t340: F, t343: F, t974: F, t2955: F, t969: F, t2967: F, t964: F, t63: F, t344: F, t221: F, t339: F) -> (F, F, F, F, F) {
    let t10327 = t340 * t10325 * t343;
    let t10328 = t974 * t10327;
    let t10331 = t2955 * t969;
    let t10333 = t964 * t2967;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = F::new(0.3086419753086419753e-3) * t339 * t10337;
    (t10328, t10331, t10333, t10335, t10339)
}
