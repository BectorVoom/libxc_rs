//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 946/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk946<F: Float>(t3856: F, t6936: F, t6943: F, t3851: F, t22827: F, t22828: F, t22817: F, t794: F, t8462: F, t1369: F, t31165: F, t3872: F, t8466: F) -> (F, F, F, F, F, F) {
    let t113972 = t6936 * t6943 * t3856;
    let t113975 = t6936 * t6943 * t3851;
    let t113978 = t22827 * t6943 * t22828;
    let t113981 = t22817 * t794 * t8462;
    let t113983 = t31165 * t1369;
    let t113985 = t8466 * t3872;
    (t113972, t113975, t113978, t113981, t113983, t113985)
}
