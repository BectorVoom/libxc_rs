//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1756;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta539<F: Float>(t1336: F, t22759: F, t835: F, t22760: F, t3777: F, t12248: F, t6604: F, t22716: F, t6983: F, t22723: F, t268: F, t534: F, t22706: F, t22863: F, t6979: F, t22641: F, t3749: F, t6978: F, t80854: F, t1984: F, t80845: F, t2010: F, t6973: F, t80742: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t80997, t81000, t81027, t81039, t81046) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1756::<F>(t1336, t22759, t835, t22760, t3777, t12248, t6604, t22716, t6983, t22723, t268, t534);
        let (t81047, t81061, t81064, t81066, t81071, t81072, t81074) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1757::<F>(t22706, t81046, t22863, t6979, t22641, t3749, t6978, t80854, t1984, t80845, t2010, t6973, t80742);
    (t80997, t81000, t81027, t81039, t81046, t81047, t81061, t81064, t81066, t81071, t81072, t81074)
}
