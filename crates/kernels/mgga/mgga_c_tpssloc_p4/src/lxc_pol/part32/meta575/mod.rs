//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta575<F: Float>(t5371: F, t7467: F, t5456: F, t576: F, t1873: F, t1458: F, t3941: F, t5493: F, t1401: F, t28017: F, t1409: F, t22510: F, t24498: F, t27356: F, t5392: F, t5398: F, t5415: F, t56: F, t7251: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t29473) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1950::<F>(t5371, t7467, t5456, t576, t1873, t1458, t3941, t5493, t1401, t28017, t1409, t22510, t24498, t27356, t5392, t5398, t5415, t56, t7251);
    (t28892, t28893, t28895, t28896, t28898, t28899, t28901, t28903, t29473)
}
