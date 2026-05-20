//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1173/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1173<F: Float>(t5903: F, t68: F, t369: F, t1539: F, t1616: F, t3071: F) -> (F, F, F, F) {
    let t5904 = t5903 * t68;
    let t5905 = t5904 * t369;
    let t5908 = t1616 * t1539;
    let t5909 = t3071 * t5908;
    (t5904, t5905, t5908, t5909)
}
