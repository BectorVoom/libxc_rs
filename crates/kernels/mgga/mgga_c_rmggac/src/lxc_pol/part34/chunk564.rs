//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 564/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk564<F: Float>(t13885: F, t13890: F, t13893: F, t302: F, t3207: F, t72: F, t14438: F, t739: F, t2106: F, t3224: F, t2145: F, t2160: F, t3180: F, t638: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14542 = F::cast_from(0.1276937996798935182e-4_f64) * t13885;
    let t14545 = F::cast_from(0.35038612185802734376e-6_f64) * t13890;
    let t14546 = F::cast_from(0.52557918278704101564e-6_f64) * t13893;
    let t14547 = t302 * t3207;
    let t14548 = t72 * t14547;
    let t14549 = t739 * t14438;
    let t14550 = F::cast_from(0.14967802127329760705e-1_f64) * t14549;
    let t14551 = t3224 * t2106;
    let t14552 = t2145 * t14551;
    let t14557 = t638 * t2160 * t3180;
    (t14542, t14545, t14546, t14547, t14548, t14550, t14551, t14552, t14557)
}
