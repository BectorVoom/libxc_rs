//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 968/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk968<F: Float>(t13141: F, t13151: F, t13157: F, t13161: F, t13164: F, t13167: F, t1504: F, t1506: F, t228: F, t230: F, t2667: F, t2672: F, t2675: F, t4219: F, t4225: F, t4227: F, t4230: F, t822: F, t825: F) -> (F,) {
    let t13170 = -t13141 * t230 - 24.0 * t13151 * t4227 + 60.0 * t13157 * t4225 - 24.0 * t13161 * t4225 - 12.0 * t13164 * t4225 + 3.0 * t13167 * t228 - 12.0 * t1504 * t2672 + 3.0 * t1504 * t2675 + 3.0 * t1506 * t2667 + 6.0 * t4219 * t825 + 6.0 * t4230 * t822;
    (t13170,)
}
