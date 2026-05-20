//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2104/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2104<F: Float>(t1888: F, t232: F, t6646: F, t87106: F, t1484: F, t852: F, t25038: F, t25248: F, t776: F, t13393: F, t22996: F, t81595: F) -> (F, F, F, F, F) {
    let t87109 = t1888 * t6646 * t87106 * t232;
    let t87111 = t852 * t1484;
    let t87114 = t25038 * t25248 * t87111 * t776;
    let t87117 = t1888 * t22996 * t13393;
    let t87119 = F::cast_from(0.16449340668482264365e-1_f64) * t81595;
    (t87109, t87111, t87114, t87117, t87119)
}
