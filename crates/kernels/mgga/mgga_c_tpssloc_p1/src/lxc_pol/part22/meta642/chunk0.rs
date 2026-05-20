//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2182/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2182<F: Float>(t12283: F, t19976: F, t19886: F, t19815: F, t3802: F, t20000: F, t54566: F, t16398: F, t19873: F, t16397: F, t5234: F, t5252: F) -> (F, F, F, F, F, F) {
    let t56837 = t12283 * t19976;
    let t56853 = t12283 * t19886;
    let t56878 = t19815 * t3802;
    let t56883 = t54566 * t20000;
    let t56885 = t16398 * t19873;
    let t56888 = t5234 * t16397 * t5252;
    (t56837, t56853, t56878, t56883, t56885, t56888)
}
