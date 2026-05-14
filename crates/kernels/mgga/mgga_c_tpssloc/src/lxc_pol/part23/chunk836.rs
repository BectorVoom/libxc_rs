//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 836/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk836<F: Float>(t1714: F, t4899: F, t11545: F, t60: F, t461: F, t11588: F, t134: F, t3439: F, t15026: F, t3032: F, t3514: F, t11147: F, t11778: F, t1742: F, t3036: F, t3503: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15390 = t4899 * t1714;
    let t15394 = t60 * t11545;
    let t15395 = t15394 * t461;
    let t15402 = t11588 * t461;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15437 = t15026 * t3032;
    let t15438 = t15437 * t3514;
    let t15453 = t11778 * t11147;
    let t15501 = t1742 * t3036;
    let t15502 = t3503 * t15501;
    (t15390, t15394, t15395, t15402, t15418, t15419, t15437, t15438, t15453, t15501, t15502)
}
