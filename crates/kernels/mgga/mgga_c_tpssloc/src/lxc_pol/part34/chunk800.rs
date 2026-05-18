//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 800/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk800<F: Float>(t1041: F, t17884: F, t248: F, t3051: F, t5681: F, t300: F, t5769: F, t2929: F, t5790: F, t1036: F, t5905: F, t4571: F, t4644: F) -> (F, F, F, F, F, F) {
    let t17885 = t1041 * t17884;
    let t17906 = t248 * t3051 * t5681;
    let t17907 = t1041 * t17906;
    let t17934 = t300 * t5769;
    let t17954 = t2929 * t5790;
    let t18005 = t5905 * t1036;
    let t18008 = t4644 * t4571;
    (t17885, t17907, t17934, t17954, t18005, t18008)
}
