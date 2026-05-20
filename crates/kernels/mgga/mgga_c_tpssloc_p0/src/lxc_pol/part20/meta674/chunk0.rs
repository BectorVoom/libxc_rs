//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2542/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2542<F: Float>(t11269: F, t1671: F, t3264: F, t11191: F, t15067: F, t43969: F, t15060: F, t3307: F, t3313: F, t11277: F, t4781: F, t11275: F, t3265: F) -> (F, F, F, F) {
    let t51453 = F::new(2.0) * t3264 * t1671 * t11269;
    let t51456 = F::cast_from(0.62071215503128080361e4_f64) * t43969 * t15067 * t11191;
    let t51459 = F::cast_from(0.48245938496077605201e2_f64) * t3313 * t15060 * t3307;
    let t51460 = t4781 * t11277;
    let t51463 = F::cast_from(0.1551780387578202009e4_f64) * t11275 * t51460 * t3265;
    (t51453, t51456, t51459, t51463)
}
