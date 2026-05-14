//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 964/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk964<F: Float>(t225: F, t7824: F, t1527: F, t7106: F, t2718: F, t7823: F, t798: F, t25211: F, t7815: F, t1528: F, t24297: F, t25206: F, t25209: F, t25214: F, t25218: F, t25226: F, t25230: F, t259: F, t2597: F, t7842: F, t855: F, t866: F) -> (F, F, F, F, F) {
    let t26700 = t7824 * t225;
    let t26702 = t7106 * t1527;
    let t26703 = t2718 * t26702;
    let t26708 = t798 * t7823;
    let t26712 = 0.38381794893125283518e-1 * t25211;
    let t26713 = t7815 * t225;
    let t26719 = -t26700 * t866 + 2.0 * t855 * t26703 + 0.82246703342411321825e-2 * t25206 - t2597 * t7842 + t26708 * t259 - t24297 * t1528 + 0.76763589786250567037e-1 * t25209 + t26712 - t26713 * t866 - 0.16449340668482264365e-1 * t25214 - 0.16449340668482264365e-1 * t25218 - 0.16449340668482264365e-1 * t25226 - 0.3289868133696452873e-1 * t25230;
    (t26700, t26703, t26708, t26713, t26719)
}
