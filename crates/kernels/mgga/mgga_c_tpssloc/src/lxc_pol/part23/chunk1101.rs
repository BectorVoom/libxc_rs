//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1101/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1101<F: Float>(t40406: F, t5202: F, t1804: F, t40005: F, t16118: F, t9577: F, t133: F, t1799: F, t40369: F, t6600: F, t12328: F, t1815: F, t12248: F, t1834: F, t111: F, t6470: F) -> (F, F, F, F, F, F, F) {
    let t54633 = t40406 * t5202;
    let t54639 = t40005 * t1804;
    let t54663 = t9577 * t16118;
    let t54725 = t40369 * t133 * t6600 * t1799;
    let t54793 = t1815 * t12328;
    let t54930 = t12248 * t1834;
    let t55388 = t6470 * t111;
    (t54633, t54639, t54663, t54725, t54793, t54930, t55388)
}
