//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2111;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2112;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2113;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta594<F: Float>(t47705: F, t2394: F, t4339: F, t4348: F, t118: F, t122: F, t154: F, t10277: F, t1043: F, t10216: F, t3061: F, t2770: F, t376: F, t1540: F, t9698: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47706, t47707) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2111::<F>(t47705, t2394, t4339);
        let t47730 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2112::<F>(t2394, t4348);
        let (t47731, t47774) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2113::<F>(t47730, t118, t122, t154);
        let (t47775, t47779, t47783, t47787) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2114::<F>(t10277, t1043, t10216, t3061, t2770, t376, t1540, t9698);
    (t47706, t47707, t47730, t47731, t47774, t47775, t47779, t47783, t47787)
}
