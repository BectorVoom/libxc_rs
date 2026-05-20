//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk714;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta137<F: Float>(t1317: F, t3726: F, t2566: F, t535: F, t795: F, t154: F, t557: F, t205: F, t1314: F, t792: F, t118: F, t1307: F, t794: F, t116: F, t534: F, t212: F, t2586: F, t1324: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3727, t3731, t3732) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk714::<F>(t1317, t3726, t2566, t535, t795, t154, t557);
        let (t3733, t3739, t3741, t3742, t3748, t3749, t3751, t3758) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk715::<F>(t205, t3732, t1314, t792, t118, t1307, t794, t116, t534, t212, t2586, t1324, t225);
    (t3727, t3731, t3732, t3733, t3739, t3741, t3742, t3748, t3749, t3751, t3758)
}
