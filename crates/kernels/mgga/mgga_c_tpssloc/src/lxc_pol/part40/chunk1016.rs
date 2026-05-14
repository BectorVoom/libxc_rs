//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1016/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1016<F: Float>(t17841: F, t340: F, t343: F, t974: F, t135: F, t5838: F, t973: F, t17801: F, t17805: F, t17809: F, t17811: F, t17814: F, t17818: F, t17821: F, t17827: F, t2960: F, t2986: F, t5839: F, t5845: F) -> (F,) {
    let t17843 = t340 * t17841 * t343;
    let t17844 = t974 * t17843;
    let t17849 = t135 * t5838;
    let t17850 = t973 * t17849;
    let t17852 = -0.27777777777777777777e-3 * t2986 * t17801 - 0.27777777777777777777e-3 * t2986 * t17805 - 0.18518518518518518518e-3 * t17809 + 0.16666666666666666666e-2 * t2986 * t17811 - 0.11111111111111111111e-2 * t2986 * t17814 + 0.55555555555555555554e-3 * t2986 * t17818 - 0.55555555555555555554e-3 * t2986 * t17821 + 0.22222222222222222222e-2 * t2960 * t5845 - 0.27777777777777777777e-3 * t17827 - 0.83333333333333333332e-3 * t973 * t17844 + 0.22222222222222222222e-2 * t2960 * t5839 - 0.27777777777777777777e-3 * t17850;
    (t17852,)
}
