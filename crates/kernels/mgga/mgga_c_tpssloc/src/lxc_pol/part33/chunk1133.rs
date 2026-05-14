//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1133/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1133<F: Float>(t23171: F, t23228: F, t7488: F, t23030: F, t25205: F, t1519: F, t212: F, t6554: F, t10143: F, t7540: F, t1625: F, t225: F, t344: F, t3173: F, t883: F, t23592: F) -> (F, F, F, F, F, F, F) {
    let t87779 = t23171 * t23228 * t7488;
    let t87898 = t23030 * t25205;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t87975 = t7540 * t10143;
    let t88050 = t344 * t1625 * t225;
    let t88076 = t3173 * t883;
    let t88138 = t23592 * t1625;
    (t87779, t87898, t87915, t87975, t88050, t88076, t88138)
}
