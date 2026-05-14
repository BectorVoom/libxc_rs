//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 842/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk842<F: Float>(t3121: F, t884: F, t3071: F, t1023: F, t2780: F, t3036: F, t67: F, t3067: F, t3186: F) -> (F, F, F, F, F, F, F) {
    let t10393 = t3121 * t884;
    let t10394 = t3071 * t10393;
    let t10397 = t1023 * t2780;
    let t10398 = t3071 * t10397;
    let t10401 = t3036 * t67;
    let t10402 = t3067 * t10401;
    let t10403 = t3186 * t10402;
    (t10393, t10394, t10397, t10398, t10401, t10402, t10403)
}
