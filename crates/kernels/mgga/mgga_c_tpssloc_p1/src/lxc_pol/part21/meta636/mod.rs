//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2421;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2422;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta636<F: Float>(t2681: F, t9671: F, t2628: F, t2690: F, t812: F, t2635: F, t2629: F, t9612: F, t2617: F, t9666: F, t2379: F, t2632: F, t6589: F, t67: F, t246: F, t2784: F, t2841: F, t22715: F, t268: F, t271: F, t2394: F, t2781: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41373, t41385, t41386, t41410, t41424, t41448) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2421::<F>(t2681, t9671, t2628, t2690, t812, t2635, t2629, t9612, t2617, t9666, t2379, t2632);
        let (t41466, t41467, t41623, t41654) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2422::<F>(t6589, t67, t246, t2784, t2841, t22715, t268, t271);
        let (t41655, t41656) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2423::<F>(t41654, t2394, t2781);
    (t41373, t41385, t41386, t41410, t41424, t41448, t41466, t41467, t41623, t41654, t41655, t41656)
}
