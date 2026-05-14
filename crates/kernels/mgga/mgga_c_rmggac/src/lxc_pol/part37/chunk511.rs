//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 511/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk511<F: Float>(t13881: F, t13885: F, t14438: F, t739: F, t2106: F, t3224: F, t2145: F, t2160: F, t3180: F, t638: F, t2164: F, t702: F, t639: F, t2231: F, t640: F, t13970: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14541 = 0.3830813990396805546e-4 * t13881;
    let t14542 = 0.1276937996798935182e-4 * t13885;
    let t14549 = t739 * t14438;
    let t14550 = 0.14967802127329760705e-1 * t14549;
    let t14551 = t3224 * t2106;
    let t14552 = t2145 * t14551;
    let t14557 = t638 * t2160 * t3180;
    let t14559 = t2164 * t702;
    let t14561 = t638 * t639 * t14559;
    let t14562 = 0.15243824895787514157e-3 * t14561;
    let t14563 = t640 * t2231;
    let t14565 = t638 * t639 * t14563;
    let t14566 = 0.15243824895787514157e-3 * t14565;
    let t14570 = 0.68186654135613354325e-2 * t13970;
    (t14541, t14542, t14550, t14551, t14552, t14557, t14559, t14562, t14563, t14566, t14570)
}
