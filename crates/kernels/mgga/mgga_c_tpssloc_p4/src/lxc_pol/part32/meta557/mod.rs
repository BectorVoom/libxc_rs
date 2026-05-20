//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1920;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta557<F: Float>(t28130: F, t6976: F, t22633: F, t19743: F, t3792: F, t22897: F, t1992: F, t6347: F, t6968: F, t6637: F, t6888: F, t6330: F, t22685: F, t1799: F, t26395: F, t6415: F, t6987: F, t1336: F, t1814: F, t2013: F, t22693: F, t26381: F, t26427: F, t27082: F, t27088: F, t6378: F, t7747: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28131, t28132, t28134, t28135, t28136, t28138, t28139, t28140, t28142) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1920::<F>(t28130, t6976, t22633, t19743, t3792, t22897, t1992, t6347, t6968, t6637, t6888, t6330);
        let (t28143, t28148, t28149, t28152, t28155) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1921::<F>(t28142, t6637, t22685, t1799, t26395, t6888, t6415, t6987, t1336, t1814, t2013, t22693, t26381, t26427, t27082, t27088, t28132, t28136, t28140, t6378, t7747);
    (t28131, t28134, t28135, t28138, t28139, t28142, t28143, t28148, t28149, t28152, t28155)
}
