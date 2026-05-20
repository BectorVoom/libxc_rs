//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1891/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1891<F: Float>(t23788: F, t86797: F, t16596: F, t83555: F, t1081: F, t4303: F, t28: F, t40772: F, t86717: F, t25365: F, t1530: F, t3231: F) -> (F, F, F, F, F, F) {
    let t89928 = t23788 * t86797;
    let t89931 = t83555 * t16596;
    let t89941 = t1081 * t4303;
    let t89953 = t40772 * t28;
    let t89954 = t89953 * t86717;
    let t89972 = t83555 * t25365;
    let t89978 = t3231 * t1530;
    (t89928, t89931, t89941, t89954, t89972, t89978)
}
