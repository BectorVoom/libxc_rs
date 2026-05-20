//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta664<F: Float>(t1081: F, t5664: F, t89953: F, t97999: F, t10143: F, t1649: F, t25374: F, t5966: F, t776: F, t4303: F, t23788: F, t67164: F) -> (F, F, F, F, F, F) {
        let (t100669, t100682, t100689, t100692, t100696, t100705) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1953::<F>(t1081, t5664, t89953, t97999, t10143, t1649, t25374, t5966, t776, t4303, t23788, t67164);
    (t100669, t100682, t100689, t100692, t100696, t100705)
}
