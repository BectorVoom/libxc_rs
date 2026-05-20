//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1778;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1779;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta458<F: Float>(t1891: F, t22822: F, t133: F, t6601: F, t6590: F, t6604: F, t13229: F, t232: F, t815: F, t22813: F, t22816: F, t1895: F, t794: F, t1899: F, t2693: F, t281: F, t6598: F, t22690: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23093, t23096, t23097) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1778::<F>(t1891, t22822, t133, t6601, t6590, t6604);
        let (t23098, t23099, t23100, t23102, t23104, t23106, t23108, t23109) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1779::<F>(t13229, t232, t815, t23097, t1891, t22813, t22816, t1895, t794, t1899, t2693, t281, t6598);
        let t23110 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1780::<F>(t22690, t814);
    (t23093, t23096, t23097, t23098, t23099, t23100, t23102, t23104, t23106, t23108, t23109, t23110)
}
