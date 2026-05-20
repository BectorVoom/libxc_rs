//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2166/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2166<F: Float>(t5154: F, t9722: F, t39659: F, t2221: F, t5166: F, t2223: F, t1788: F, t9216: F, t9218: F, t39855: F, t39857: F, t9494: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54451 = t5154 * t9722;
    let t54453 = F::new(96.0) * t39659;
    let t54456 = t2221 * t5166;
    let t54457 = F::new(36.0) * t54456;
    let t54459 = F::new(96.0) * t2223 * t5166;
    let t54460 = t9216 * t1788;
    let t54461 = F::new(240.0) * t54460;
    let t54462 = t9218 * t1788;
    let t54465 = F::new(480.0) * t39855;
    let t54466 = F::new(96.0) * t39857;
    let t54467 = t5154 * t9494;
    (t54451, t54453, t54457, t54459, t54461, t54462, t54465, t54466, t54467)
}
