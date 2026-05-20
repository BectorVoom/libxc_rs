//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1914/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1914<F: Float>(t22986: F, t25054: F, t86873: F, t6552: F, t6555: F, t98133: F, t1880: F, t25216: F, t25224: F, t25038: F, t25040: F, t28267: F, t81651: F, t82074: F) -> (F, F, F, F, F) {
    let t98196 = t22986 * t86873 * t25054;
    let t98199 = t6552 * t98133 * t6555;
    let t98202 = t1880 * t25224 * t25216;
    let t98205 = t25038 * t86873 * t25040;
    let t98213 = t81651 * t82074 * t28267;
    (t98196, t98199, t98202, t98205, t98213)
}
