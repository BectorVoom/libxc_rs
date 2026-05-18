//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 885/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk885<F: Float>(t11328: F, t11343: F, t1137: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t11306: F, t1147: F, t3368: F, t1143: F, t3400: F) -> (F, F, F, F, F) {
    let t11344 = t11328 + t11343;
    let t11345 = t11344 * t1137;
    let t11349 = F::new(1.0) / t3355 / t1127;
    let t11350 = t427 * t11349;
    let t11352 = F::new(1.0) / t3358 / t435;
    let t11353 = t11306 * t11352;
    let t11356 = t3368 * t1147;
    let t11361 = t1143 * t3400;
    (t11345, t11350, t11353, t11356, t11361)
}
