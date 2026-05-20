//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1391;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1392;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1393;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1394;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1395;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta259<F: Float>(t10401: F, t3575: F, t3610: F, t3624: F, t3521: F, t820: F, t1190: F, t3030: F, t3032: F, t3505: F, t10469: F, t466: F, t10471: F) -> (F, F, F, F, F, F, F, F, F) {
        let t11677 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1391::<F>(t10401, t3575);
        let t11678 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1392::<F>(t11677, t3610);
        let t11692 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1393::<F>(t11677, t3624);
        let t11697 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1394::<F>(t3521, t820);
        let (t11707, t11708, t11709, t11712) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1395::<F>(t1190, t3030, t3032, t3505, t10469, t466);
        let t11713 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1396::<F>(t10471, t11712);
    (t11677, t11678, t11692, t11697, t11707, t11708, t11709, t11712, t11713)
}
