//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1192/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1192<F: Float>(t15971: F, t592: F, t2221: F, t5168: F, t2225: F, t5154: F, t9892: F, t9722: F, t1788: F, t9216: F, t9218: F, t9494: F) -> (F, F, F, F, F, F, F, F) {
    let t54412 = t592 * t15971;
    let t54428 = t2221 * t5168;
    let t54432 = t2225 * t5168;
    let t54434 = t5154 * t9892;
    let t54451 = t5154 * t9722;
    let t54460 = t9216 * t1788;
    let t54462 = t9218 * t1788;
    let t54467 = t5154 * t9494;
    (t54412, t54428, t54432, t54434, t54451, t54460, t54462, t54467)
}
