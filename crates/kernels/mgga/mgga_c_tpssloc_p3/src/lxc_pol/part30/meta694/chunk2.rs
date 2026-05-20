//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2216/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2216<F: Float>(t1527: F, t22986: F, t23270: F, t86849: F, t4272: F, t86969: F, t1520: F, t254: F, t25038: F, t25039: F, t4119: F, t1880: F, t7488: F, t87782: F) -> (F, F, F, F, F) {
    let t98264 = t22986 * t23270 * t86849 * t1527;
    let t98277 = t22986 * t23270 * t86969 * t4272;
    let t98279 = t1520 * t254;
    let t98291 = t25038 * t23270 * t25039 * t4119;
    let t98305 = t1880 * t87782 * t7488;
    (t98264, t98277, t98279, t98291, t98305)
}
