//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 721/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk721<F: Float>(t1986: F, t3141: F, t9189: F, t1602: F, t793: F, t14374: F, t15318: F, t14363: F, t15322: F, t118: F, t128: F, t1392: F, t14011: F, t14047: F, t11654: F, t14236: F, t14249: F, t2078: F) -> (F, F, F, F, F, F, F) {
    let t75016 = t3141 * t1986 * t9189;
    let t75020 = t3141 * t1986 * t793 * t1602;
    let t75022 = t14374 * t15318;
    let t75024 = t14363 * t15322;
    let t75027 = t118 * t128 * t1392;
    let t75029 = t14047 * t14011 * t75027;
    let t75033 = t14236 * t14249 * t2078 * t11654;
    (t75016, t75020, t75022, t75024, t75027, t75029, t75033)
}
