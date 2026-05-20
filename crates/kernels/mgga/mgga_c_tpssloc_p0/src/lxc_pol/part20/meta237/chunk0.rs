//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1336/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1336<F: Float>(t1891: F, t67: F, t246: F, t232: F, t2379: F, t2646: F, t2645: F, t2647: F, t9626: F, t210: F, t2553: F, t804: F) -> (F, F, F, F, F, F) {
    let t9645 = t1891 * t67;
    let t9646 = t9645 * t246;
    let t9647 = t232 * t2379;
    let t9649 = t9646 * t2646 * t9647;
    let t9653 = t2645 * t9626 * t2647;
    let t9657 = t210 * t804 * t2553;
    (t9645, t9646, t9647, t9649, t9653, t9657)
}
