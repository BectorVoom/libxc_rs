//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1024/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1024<F: Float>(t23511: F, t363: F, t1014: F, t3030: F, t3127: F, t3037: F, t3033: F, t6753: F, t3: F, t6740: F, t2978: F, t344: F, t381: F, t3034: F, t38: F, t131: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23512 = t23511 * t363;
    let t23518 = t3030 * t1014;
    let t23519 = t23518 * t363;
    let t23535 = t3127 * sigma0;
    let t23536 = t23535 * t3037;
    let t23537 = t3033 * t23536;
    let t23540 = t6753 * t3037;
    let t23541 = t3033 * t23540;
    let t23562 = t6740 * t3;
    let t23592 = t2978 * t344;
    let t23593 = t23592 * t381;
    let t23598 = 1.0 / t3034;
    let t23599 = t38 * t23598;
    let t23600 = t23599 * t131;
    (t23512, t23518, t23519, t23535, t23536, t23537, t23540, t23541, t23562, t23592, t23593, t23599, t23600)
}
