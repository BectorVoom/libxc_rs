//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 820/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk820<F: Float>(t22716: F, t8459: F, t31170: F, t3809: F, t22779: F, t31162: F, t22759: F, t3793: F, t6936: F, t3856: F, t6943: F, t3851: F, t22827: F, t22828: F, t22817: F, t794: F, t8462: F) -> (F, F, F, F, F, F, F, F) {
    let t113963 = 0.12793931631041761173e0 * t22716 * t8459;
    let t113964 = t31170 * t3809;
    let t113966 = t22779 * t31162;
    let t113969 = t6936 * t22759 * t3793;
    let t113972 = t6936 * t6943 * t3856;
    let t113975 = t6936 * t6943 * t3851;
    let t113978 = t22827 * t6943 * t22828;
    let t113981 = t22817 * t794 * t8462;
    (t113963, t113964, t113966, t113969, t113972, t113975, t113978, t113981)
}
