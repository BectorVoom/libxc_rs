//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2029/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2029<F: Float>(t225: F, t2627: F, t236: F, t25093: F, t87229: F, t1512: F, t81807: F, t81824: F, t23041: F, t4236: F, t23040: F, t4166: F) -> (F, F, F, F, F, F) {
    let t87230 = t225 * t2627;
    let t87233 = t87229 * t87230 * t236 * t25093;
    let t87234 = F::cast_from(0.13457585364713463618e-3_f64) * t87233;
    let t87243 = t81807 * t1512;
    let t87247 = t81824 * t1512;
    let t87248 = F::new(7.0) / F::new(1152.0) * t87247;
    let t87255 = t23041 * t4236;
    let t87256 = F::new(7.0) / F::new(1152.0) * t87255;
    let t87261 = t4166 * t23040;
    (t87230, t87234, t87243, t87248, t87256, t87261)
}
