//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1357/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1357<F: Float>(t10965: F, t3053: F, t3082: F, t3094: F, t10895: F, t10952: F, t1022: F, t3120: F, t2250: F, t360: F, t1036: F, t10367: F) -> (F, F, F, F, F, F) {
    let t43226 = t10965 * t3053;
    let t43228 = t3094 * t3082;
    let t43233 = t10952 * t10895;
    let t43235 = t3120 * t1022;
    let t43240 = t2250 * t1022;
    let t43241 = t43240 * t360;
    let t43246 = t10367 * t1036;
    (t43226, t43228, t43233, t43235, t43241, t43246)
}
