//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2589/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2589<F: Float>(t71821: F, t71850: F, t71853: F, t71855: F, t71867: F, t72081: F, t72083: F, t72086: F, t72094: F, t72096: F, t72195: F, t72196: F, t72198: F, t72214: F) -> F {
    let t72217 = t72195 + t72196 + t72198 - t71821 - t71850 + t71853 - t71855 - t72081 + t71867 + t72083 - t72086 - t72094 + t72096 + t72214;
    t72217
}
