//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2483/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2483<F: Float>(t1667: F, t9709: F, t14712: F, t699: F, t1113: F, t136: F, t50830: F, t2403: F, t4778: F, t4723: F, t9258: F, t3297: F) -> (F, F, F, F, F, F, F) {
    let t50846 = t9709 * t1667;
    let t50848 = t699 * t14712;
    let t50851 = t136 * t1113 * t50830;
    let t50853 = t2403 * t4778;
    let t50854 = F::cast_from(0.27595e0_f64) * t50853;
    let t50857 = t4723 * t9258;
    let t50859 = t136 * t3297 * t50857;
    (t50846, t50848, t50851, t50853, t50854, t50857, t50859)
}
