//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2592/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2592<F: Float>(t1227: F, t21776: F, t248: F, t3521: F, t18392: F, t5005: F, t15737: F, t18356: F, t19040: F, t5024: F, t11738: F, t22299: F, t3570: F) -> (F, F, F, F, F) {
    let t72273 = t1227 * t248 * t3521 * t21776;
    let t72285 = t5005 * t18392;
    let t72287 = t15737 * t18356;
    let t72289 = t5024 * t19040;
    let t72293 = t11738 * t248 * t3570 * t22299;
    (t72273, t72285, t72287, t72289, t72293)
}
