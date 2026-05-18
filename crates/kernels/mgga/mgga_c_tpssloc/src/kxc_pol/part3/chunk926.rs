//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 926/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk926<F: Float>(t2929: F, t938: F, t10523: F, t315: F, t10544: F, t1004: F, t3047: F, t3053: F, t3117: F, t1043: F, t676: F, t248: F, t884: F) -> (F, F, F, F, F, F, F) {
    let t10825 = t938 * t2929;
    let t10828 = t315 * t10523;
    let t10832 = F::new(0.53272592592592592592e-1) * t10544;
    let t10863 = t1004 * t3047;
    let t10866 = t3117 * t3053;
    let t10868 = t676 * t1043;
    let t10870 = t248 * t10868 * t884;
    (t10825, t10828, t10832, t10863, t10866, t10868, t10870)
}
