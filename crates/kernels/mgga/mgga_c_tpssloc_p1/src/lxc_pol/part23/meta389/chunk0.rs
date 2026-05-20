//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1193/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1193<F: Float>(t15971: F, t588: F, t12364: F, t5234: F, t1811: F, t40005: F, t40406: F, t5202: F, t1804: F, t16118: F, t9577: F, t133: F, t1799: F, t40369: F, t6600: F) -> (F, F, F, F, F, F, F) {
    let t54477 = t588 * t15971;
    let t54532 = t5234 * t12364;
    let t54582 = t40005 * t1811;
    let t54633 = t40406 * t5202;
    let t54639 = t40005 * t1804;
    let t54663 = t9577 * t16118;
    let t54725 = t40369 * t133 * t6600 * t1799;
    (t54477, t54532, t54582, t54633, t54639, t54663, t54725)
}
