//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta446 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1594;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1595;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta446<F: Float>(t23230: F, t225: F, t7072: F, t7085: F, t23251: F, t23261: F, t2752: F, t7109: F, t10143: F, t2056: F, t2094: F, t3701: F) -> (F, F, F, F, F, F, F, F) {
        let (t24291, t24297, t24305, t24318, t24321, t24339) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1594::<F>(t23230, t225, t7072, t7085, t23251, t23261, t2752, t7109);
        let t24344 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1595::<F>(t10143, t2056);
        let t24432 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1596::<F>(t2094, t3701);
    (t24291, t24297, t24305, t24318, t24321, t24339, t24344, t24432)
}
