//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 829/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk829<F: Float>(t30706: F, t6605: F, t808: F, t8342: F, t8344: F, t240: F, t241: F, t814: F, t812: F, t232: F, t2646: F, t4180: F) -> (F, F, F, F, F, F) {
    let t30707 = t6605 * t30706;
    let t30709 = t808 * t8342;
    let t30710 = t30709 * t8344;
    let t30713 = t814 * t240 * t241;
    let t30714 = t812 * t30713;
    let t30716 = t4180 * t2646 * t232;
    (t30707, t30709, t30710, t30713, t30714, t30716)
}
