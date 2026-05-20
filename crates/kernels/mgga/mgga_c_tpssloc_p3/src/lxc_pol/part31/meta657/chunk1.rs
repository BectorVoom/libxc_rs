//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1941/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1941<F: Float>(t23109: F, t2632: F, t81914: F, t98779: F, t23110: F, t232: F, t236: F, t5611: F, t5587: F, t81886: F, t23041: F, t5619: F) -> (F, F, F, F, F) {
    let t98782 = t23109 * t81914 * t98779 * t2632;
    let t98787 = t23109 * t23110 * t236 * t5611 * t232;
    let t98791 = t23109 * t23110 * t98779 * t232;
    let t98796 = t81886 * t5587;
    let t98798 = t23041 * t5619;
    (t98782, t98787, t98791, t98796, t98798)
}
