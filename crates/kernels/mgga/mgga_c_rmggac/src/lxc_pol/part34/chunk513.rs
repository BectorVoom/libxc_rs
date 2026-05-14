//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 513/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk513<F: Float>(t14547: F, t72: F, t14438: F, t739: F, t2106: F, t3224: F, t2145: F, t2160: F, t3180: F, t638: F, t2164: F, t702: F, t639: F, t2231: F, t640: F, t290: F, t3207: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14548 = t72 * t14547;
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
    let t14567 = t290 * t3207;
    (t14548, t14550, t14551, t14552, t14557, t14559, t14562, t14563, t14566, t14567)
}
