//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1779/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1779<F: Float>(t1883: F, t82045: F, t23012: F, t6568: F, t23205: F, t82038: F, t1081: F, t2752: F, t608: F, t9239: F, t22573: F, t6875: F) -> (F, F, F, F, F, F) {
    let t82218 = t82045 * t1883;
    let t82259 = t23012 * t6568;
    let t82294 = t82038 * t23205;
    let t83555 = t2752 * t1081;
    let t83717 = t9239 * t608;
    let t83886 = t6875 * t22573;
    (t82218, t82259, t82294, t83555, t83717, t83886)
}
