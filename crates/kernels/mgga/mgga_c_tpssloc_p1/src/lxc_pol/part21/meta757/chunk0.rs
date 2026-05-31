//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2631/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2631<F: Float>(t15977: F, t588: F, t5157: F, t9874: F, t5137: F, t591: F, t5145: F, t15908: F, t9885: F, t9888: F, t15968: F, t172: F, t763: F) -> (F, F, F, F, F, F, F) {
    let t54323 = t588 * t15977;
    let t54325 = t5157 * t9874;
    let t54347 = F::cast_from(32.0_f64) * t5137 * t591;
    let t54370 = F::cast_from(32.0_f64) * t5145 * t591;
    let t54380 = t15908 * t9885;
    let t54382 = t15908 * t9888;
    let t54387 = t15968 * t172 * t763;
    (t54323, t54325, t54347, t54370, t54380, t54382, t54387)
}
