//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta615<F: Float>(t23563: F, t6740: F, t10922: F, t6717: F, t3200: F, t83015: F, t1030: F, t1058: F, t3068: F, t25511: F, t6743: F, t23592: F, t23631: F, t974: F, sigma0: F) -> (F, F, F, F, F, F) {
        let (t83138, t83157, t83215, t83220, t83233, t83239) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2014::<F>(t23563, t6740, t10922, t6717, t3200, t83015, t1030, t1058, t3068, t25511, t6743, t23592, t23631, t974, sigma0);
    (t83138, t83157, t83215, t83220, t83233, t83239)
}
