//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk635;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk636;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk637;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk638;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta109<F: Float>(t154: F, t2559: F, t222: F, t2563: F, t805: F, t68: F, t808: F, t816: F, t809: F, t838: F, t842: F, t233: F, t813: F, t236: F, t240: F, t812: F, t232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2600, t2602, t2603, t2617) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk635::<F>(t154, t2559, t222, t2563, t805, t68, t808);
        let (t2618, t2621, t2623, t2627) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk636::<F>(t2617, t816, t809, t838, t842, t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk637::<F>(t236, t2627);
        let (t2629, t2630, t2632) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk638::<F>(t240, t2628, t812, t232);
    (t2600, t2602, t2603, t2617, t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2632)
}
