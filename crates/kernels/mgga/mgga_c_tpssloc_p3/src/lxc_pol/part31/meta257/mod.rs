//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta257 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1074;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1075;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1076;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1077;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta257<F: Float>(t1860: F, t7032: F, t2031: F, t6509: F, t5: F, t2032: F, t6486: F, t6492: F, t6495: F, t7026: F, t112: F, t111: F, t2035: F, t1266: F, t2039: F, t109: F, t6528: F, t6531: F, t510: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7034, t7035) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1074::<F>(t1860, t7032, t2031, t6509);
        let (t7039, t7040, t7042) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1075::<F>(t5, t1860, t2032, t6486, t6492, t6495, t7026, t7034, t7035, t112, t111, t2035);
        let t7050 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1076::<F>(t1266, t2039);
        let (t7053, t7056) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1077::<F>(t109, t6528, t6531);
        let t7057 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1078::<F>(t510, t7056);
    (t7034, t7035, t7039, t7040, t7042, t7050, t7053, t7056, t7057)
}
