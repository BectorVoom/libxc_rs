//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2517;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2518;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2519;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2520;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2521;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta694<F: Float>(t2394: F, t4348: F, t13612: F, t690: F, t13580: F, t118: F, t122: F, t154: F, t10277: F, t1043: F, t10216: F, t3061: F, t2770: F, t376: F, t1540: F, t9698: F) -> (F, F, F, F, F, F, F, F) {
        let t47730 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2517::<F>(t2394, t4348);
        let t47732 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2518::<F>(t13612, t690);
        let t47738 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2519::<F>(t13580, t690);
        let t47774 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2520::<F>(t118, t122, t154);
        let (t47775, t47779, t47783, t47787) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2521::<F>(t10277, t1043, t10216, t3061, t2770, t376, t1540, t9698);
    (t47730, t47732, t47738, t47774, t47775, t47779, t47783, t47787)
}
