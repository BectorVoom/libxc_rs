//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1447;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1448;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta291<F: Float>(t2770: F, t3966: F, t10216: F, t1409: F, t2775: F, t4389: F, t699: F, t4386: F, t10277: F, t4339: F, t690: F, t4344: F, t1540: F, t2394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13527, t13536, t13541, t13550, t13551, t13552, t13554, t13563) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1447::<F>(t2770, t3966, t10216, t1409, t2775, t4389, t699, t4386, t10277, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1448::<F>(t4344, t690);
        let (t13567, t13598) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1449::<F>(t13566, t1540, t2394);
    (t13527, t13536, t13541, t13550, t13551, t13552, t13554, t13563, t13566, t13567, t13598)
}
