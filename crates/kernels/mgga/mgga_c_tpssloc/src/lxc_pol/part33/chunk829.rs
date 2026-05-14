//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 829/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk829<F: Float>(t135: F, t5844: F, t973: F, t5838: F, t10236: F, t5392: F, t10457: F, t248: F, t5677: F, t1041: F, t3051: F, t5681: F, t300: F, t5769: F, t2929: F, t5790: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17826 = t135 * t5844;
    let t17827 = t973 * t17826;
    let t17849 = t135 * t5838;
    let t17850 = t973 * t17849;
    let t17863 = t10236 * t5392;
    let t17884 = t248 * t10457 * t5677;
    let t17885 = t1041 * t17884;
    let t17906 = t248 * t3051 * t5681;
    let t17907 = t1041 * t17906;
    let t17934 = t300 * t5769;
    let t17954 = t2929 * t5790;
    (t17827, t17850, t17863, t17884, t17885, t17906, t17907, t17934, t17954)
}
