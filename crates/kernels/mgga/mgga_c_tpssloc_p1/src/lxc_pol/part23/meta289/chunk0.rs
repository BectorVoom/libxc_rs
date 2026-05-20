//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1000/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1000<F: Float>(t21138: F, t908: F, t136: F, t4362: F, t5705: F, t4378: F, t10564: F, t21130: F, t123: F) -> (F, F, F, F, F, F) {
    let t21139 = t908 * t21138;
    let t21140 = t136 * t21139;
    let t21142 = t4362 * t5705;
    let t21144 = t4378 * t5705;
    let t21146 = t10564 * t21130;
    let t21147 = t123 * t21146;
    (t21139, t21140, t21142, t21144, t21146, t21147)
}
