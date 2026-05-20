//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1414;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1415;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1416;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta270<F: Float>(t12189: F, t1317: F, t535: F, t795: F, t9580: F, t3749: F, t9577: F, t1314: F, t2566: F, t3741: F, t3732: F, t792: F, t782: F, t1365: F, t154: F) -> (F, F, F, F, F, F, F, F) {
        let (t12190, t12194, t12196, t12199, t12200, t12202) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1414::<F>(t12189, t1317, t535, t795, t9580, t3749, t9577, t1314, t2566, t3741, t3732, t792);
        let t12211 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1415::<F>(t3732, t782);
        let t12214 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1416::<F>(t1365, t154);
    (t12190, t12194, t12196, t12199, t12200, t12202, t12211, t12214)
}
