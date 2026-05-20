//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 935/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk935<F: Float>(t2791: F, t888: F, t2929: F, t938: F, t10523: F, t315: F, t10544: F, t1043: F, t676: F, t248: F, t884: F, t1041: F) -> (F, F, F, F, F, F) {
    let t10817 = t888 * t2791;
    let t10825 = t938 * t2929;
    let t10828 = t315 * t10523;
    let t10832 = F::cast_from(0.53272592592592592592e-1_f64) * t10544;
    let t10868 = t676 * t1043;
    let t10870 = t248 * t10868 * t884;
    let t10871 = t1041 * t10870;
    (t10817, t10825, t10828, t10832, t10868, t10871)
}
