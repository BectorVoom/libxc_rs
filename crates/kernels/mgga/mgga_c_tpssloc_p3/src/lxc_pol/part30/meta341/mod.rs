//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta341<F: Float>(t4250: F, t9638: F, t4240: F, t4191: F, t2697: F, t4261: F, t820: F, t9645: F, t1484: F, t828: F, t1516: F, t9993: F) -> (F, F, F, F, F, F, F) {
        let (t13287, t13320, t13330, t13345, t13350, t13351, t13359) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1375::<F>(t4250, t9638, t4240, t4191, t2697, t4261, t820, t9645, t1484, t828, t1516, t9993);
    (t13287, t13320, t13330, t13345, t13350, t13351, t13359)
}
