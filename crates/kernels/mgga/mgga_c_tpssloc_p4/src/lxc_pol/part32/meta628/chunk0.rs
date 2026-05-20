//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2038/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2038<F: Float>(t23012: F, t7529: F, t23110: F, t23185: F, t25241: F, t1484: F, t852: F, t252: F, t4119: F, t25160: F, t814: F, t22690: F, t7520: F, t81573: F) -> (F, F, F, F, F, F) {
    let t87080 = t23012 * t7529;
    let t87100 = t23185 * t23110 * t25241;
    let t87101 = F::cast_from(0.82246703342411321824e-2_f64) * t87100;
    let t87111 = t852 * t1484;
    let t87130 = t252 * t4119;
    let t87135 = t814 * t25160;
    let t87140 = t81573 * t22690 * t7520;
    (t87080, t87101, t87111, t87130, t87135, t87140)
}
