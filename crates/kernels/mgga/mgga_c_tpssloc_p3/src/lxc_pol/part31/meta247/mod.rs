//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta247 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1040;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1041;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1042;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta247<F: Float>(t815: F, t829: F, t6605: F, t1898: F, t808: F, t249: F, t59: F, t814: F, t240: F, t812: F, t831: F, t1899: F, t838: F, t234: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6606, t6607, t6609, t6610, t6612) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1039::<F>(t815, t829, t6605, t1898, t808, t249, t59, t814);
        let t6613 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1040::<F>(t240, t6612);
        let t6614 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1041::<F>(t6613, t812);
        let (t6615, t6617, t6619, t6620) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1042::<F>(t6614, t831, t1899, t838, t234, t59, t240);
        let t6621 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1043::<F>(t6620, t812);
    (t6606, t6607, t6609, t6610, t6612, t6613, t6614, t6615, t6617, t6619, t6620, t6621)
}
