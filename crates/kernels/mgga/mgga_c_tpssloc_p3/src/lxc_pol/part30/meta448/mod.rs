//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1707;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta448<F: Float>(t1329: F, t22797: F, t2230: F, t6924: F, t213: F, t6928: F, t10: F, t2229: F, t60: F, t1995: F, t116: F, t117: F, t67: F) -> (F, F, F, F, F, F, F, F) {
        let (t22798, t22803, t22804, t22805, t22811, t22813, t22814, t22815) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1707::<F>(t1329, t22797, t2230, t6924, t213, t6928, t10, t2229, t60, t1995, t116, t117);
        let t22816 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1708::<F>(t22815, t67);
    (t22798, t22803, t22804, t22805, t22811, t22813, t22814, t22816)
}
