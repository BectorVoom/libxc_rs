//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1940;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta657<F: Float>(t232: F, t25119: F, t58557: F, t815: F, t22690: F, t5527: F, t81792: F, t841: F, t16805: F, t1898: F, t249: F, t236: F, t5584: F, t23109: F, t2632: F, t81914: F, t23110: F, t5611: F, t5587: F, t81886: F, t23041: F, t5619: F) -> (F, F, F, F, F, F, F, F) {
        let (t98770, t98774, t98777, t98779) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1940::<F>(t232, t25119, t58557, t815, t22690, t5527, t81792, t841, t16805, t1898, t249, t236, t5584);
        let (t98782, t98787, t98791, t98796, t98798) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1941::<F>(t23109, t2632, t81914, t98779, t23110, t232, t236, t5611, t5587, t81886, t23041, t5619);
    (t98770, t98774, t98777, t98782, t98787, t98791, t98796, t98798)
}
