//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1014/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1014<F: Float>(t13804: F, t13845: F, t13894: F, t13937: F, t225: F, t68: F, t369: F, t1036: F, t4622: F, t3117: F, t4571: F, t248: F, t3051: F, t4347: F, t1041: F, t10370: F, t10372: F, t10377: F, t10381: F, t10385: F, t10390: F, t13750: F, t13751: F, t13758: F, t13762: F, t13767: F, t3070: F, t378: F, t4579: F) -> (F, F, F) {
    let t13939 = t13804 + t13845 + t13894 + t13937;
    let t13940 = t13939 * t225;
    let t13941 = t13940 * t68;
    let t13942 = t13941 * t369;
    let t13946 = t4622 * t1036 / 432.0;
    let t13948 = t3117 * t4571 / 3456.0;
    let t13950 = t248 * t3051 * t4347;
    let t13952 = t1041 * t13950 / 3456.0;
    let t13953 = -t13750 + 19.0 / 1728.0 * t13751 * t378 + t10370 / 4608.0 + t10372 / 1296.0 + t10377 + t10381 / 81.0 + t10385 + t13758 + t10390 * t4579 / 2304.0 + t3070 * t13762 / 2304.0 + t13767 + t13942 * t378 / 3072.0 - t13946 + t13948 + t13952;
    (t13939, t13940, t13953)
}
