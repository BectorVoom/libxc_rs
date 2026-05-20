//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1729;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1730;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta457<F: Float>(t1891: F, t22813: F, t22816: F, t1895: F, t794: F, t1899: F, t2693: F, t281: F, t6598: F, t22690: F, t814: F, t232: F, t236: F, t828: F, t6609: F, t838: F, t6589: F, t6597: F, t776: F, t841: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23102, t23104, t23106, t23108, t23109) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1729::<F>(t1891, t22813, t22816, t1895, t794, t1899, t2693, t281, t6598);
        let t23110 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1730::<F>(t22690, t814);
        let (t23113, t23114, t23119, t23121, t23122, t23124) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1731::<F>(t23110, t232, t236, t828, t23109, t6609, t838, t6589, t6597, t281, t22690, t776, t841);
    (t23102, t23104, t23106, t23108, t23109, t23110, t23113, t23114, t23119, t23121, t23122, t23124)
}
