//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2041/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2041<F: Float>(t87211: F, t25132: F, t81876: F, t131: F, t6598: F, t9537: F, t225: F, t2627: F, t236: F, t25093: F, t1512: F, t81807: F) -> (F, F, F, F, F, F) {
    let t87212 = F::cast_from(0.6728792682356731809e-4_f64) * t87211;
    let t87213 = t81876 * t25132;
    let t87229 = t6598 * t131 * t9537;
    let t87230 = t225 * t2627;
    let t87233 = t87229 * t87230 * t236 * t25093;
    let t87234 = F::cast_from(0.13457585364713463618e-3_f64) * t87233;
    let t87243 = t81807 * t1512;
    (t87212, t87213, t87229, t87230, t87234, t87243)
}
