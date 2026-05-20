//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2103/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2103<F: Float>(t1888: F, t232: F, t47448: F, t6646: F, t23110: F, t23185: F, t25241: F, t25038: F, t25248: F, t25249: F, t2553: F, t1519: F, t2631: F) -> (F, F, F, F) {
    let t87097 = t1888 * t6646 * t47448 * t232;
    let t87100 = t23185 * t23110 * t25241;
    let t87101 = F::cast_from(0.82246703342411321824e-2_f64) * t87100;
    let t87104 = t25038 * t25248 * t25249 * t2553;
    let t87106 = t1519 * t2631;
    (t87097, t87101, t87104, t87106)
}
