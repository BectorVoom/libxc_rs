//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1084/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1084<F: Float>(t82218: F, t10115: F, t24282: F, t25168: F, t2597: F, t26728: F, t82211: F, t82221: F, t82228: F, t82230: F, t82233: F, t82236: F, t82255: F, t82259: F, t225: F, t24237: F) -> (F, F) {
    let t85129 = 0.55440370401180965083e0 * t82218;
    let t85142 = -0.38381794893125283518e0 * t82211 - t85129 + 0.9869604401089358619e-1 * t82221 - 0.29608813203268075857e0 * t82228 - 0.23029076935875170111e0 * t82230 - 0.9869604401089358619e-1 * t82233 - 0.24674011002723396548e-1 * t82236 - 18.0 * t25168 * t26728 * t10115 - 0.9869604401089358619e-1 * t82255 + 0.38381794893125283518e0 * t82259 - 3.0 * t2597 * t24282;
    let t85146 = t24237 * t225;
    (t85142, t85146)
}
