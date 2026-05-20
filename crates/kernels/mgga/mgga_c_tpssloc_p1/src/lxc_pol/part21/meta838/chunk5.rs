//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2995/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2995<F: Float>(t59637: F, t60810: F, t60812: F, t60814: F, t60816: F, t60821: F, t60825: F, t60827: F, t60829: F, t60831: F, t60834: F, t60836: F) -> F {
    let t62729 = t60810 - t60812 - t60814 + t60816 - t60821 - t60825 - t59637 - t60827 + t60829 - t60831 - t60834 - t60836;
    t62729
}
