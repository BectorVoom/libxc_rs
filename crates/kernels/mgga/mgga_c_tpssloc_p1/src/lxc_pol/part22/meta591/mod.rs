//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2106;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta591<F: Float>(t116: F, t212: F, t2570: F, t2585: F, t4255: F, t2628: F, t2691: F, t4184: F, t812: F, t1512: F, t41362: F, t13176: F, t2629: F, t4166: F, t9666: F, t2693: F, t4163: F, t41008: F, t4155: F, t41115: F, t4240: F, t41340: F, t4236: F, t9671: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46853, t46856, t46875, t46876, t46878) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2106::<F>(t116, t212, t2570, t2585, t4255, t2628, t2691, t4184, t812, t1512, t41362, t13176, t2629);
        let (t46881, t46887, t46912, t46929, t46952, t46953) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2107::<F>(t4166, t9666, t2693, t4163, t41008, t4155, t41115, t4240, t1512, t41340, t4236, t9671);
    (t46853, t46856, t46875, t46876, t46878, t46881, t46887, t46912, t46929, t46952, t46953)
}
