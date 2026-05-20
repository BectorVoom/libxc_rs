//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2194;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta609<F: Float>(t11677: F, t11904: F, t11702: F, t3536: F, t11709: F, t11745: F, t11651: F, t11734: F, t1174: F, t3556: F, t698: F, t11844: F, t135: F, t11849: F, t11153: F, t1176: F, t11881: F, t45113: F, t11773: F, t1227: F, t13969: F, t11168: F, t3431: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t45162, t45167, t45169, t45171, t45178, t45181) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2194::<F>(t11677, t11904, t11702, t3536, t11709, t11745, t11651, t11734, t1174, t3556, t698, t11844, t135);
        let (t45184, t45192, t45197, t45211, t45222) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2195::<F>(t1174, t11849, t135, t11153, t1176, t11881, t45113, t11773, t1227, t13969, t11168, t3431);
    (t45162, t45167, t45169, t45171, t45178, t45181, t45184, t45192, t45197, t45211, t45222)
}
