//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 939/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk939<F: Float>(t114299: F, t114172: F, t22892: F, t6891: F, t1307: F, t6995: F, t31236: F, t31238: F, t8326: F, t9348: F, t12734: F, t12739: F) -> (F, F, F, F, F, F, F, F) {
    let t114300 = F::cast_from(0.16449340668482264365e-1_f64) * t114299;
    let t114316 = t22892 * t114172 * t6891;
    let t114317 = F::cast_from(0.3289868133696452873e-1_f64) * t114316;
    let t114335 = t1307 * t6995;
    let t114387 = F::new(4.0) * t31236;
    let t114388 = F::new(4.0) * t31238;
    let t114405 = F::new(2.0) * t9348 * t8326;
    let t114413 = F::new(4.0) * t12734 * t8326;
    let t114415 = F::new(2.0) * t12739 * t8326;
    (t114300, t114317, t114335, t114387, t114388, t114405, t114413, t114415)
}
