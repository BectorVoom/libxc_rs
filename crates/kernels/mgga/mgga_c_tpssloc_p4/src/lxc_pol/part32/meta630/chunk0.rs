//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2042/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2042<F: Float>(t1512: F, t81824: F, t23041: F, t4236: F, t23040: F, t4166: F, t831: F, t4191: F, t81749: F, t4240: F, t23069: F, t4159: F) -> (F, F, F, F, F, F, F) {
    let t87247 = t81824 * t1512;
    let t87248 = F::new(7.0) / F::new(1152.0) * t87247;
    let t87255 = t23041 * t4236;
    let t87256 = F::new(7.0) / F::new(1152.0) * t87255;
    let t87261 = t4166 * t23040;
    let t87262 = t87261 * t831;
    let t87263 = F::new(7.0) / F::new(1152.0) * t87262;
    let t87270 = t81749 * t4191;
    let t87271 = F::new(7.0) / F::new(288.0) * t87270;
    let t87272 = t81749 * t4240;
    let t87273 = F::new(7.0) / F::new(1152.0) * t87272;
    let t87291 = t23069 * t4159;
    (t87248, t87256, t87261, t87263, t87271, t87273, t87291)
}
