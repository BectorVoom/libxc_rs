//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1254/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1254<F: Float>(t113981: F, t1369: F, t31176: F, t22804: F, t31156: F, t31169: F, t3777: F, t1336: F, t1338: F, t241: F, t835: F, t31172: F) -> (F, F, F, F, F, F) {
    let t113982 = F::cast_from(0.6728792682356731809e-4_f64) * t113981;
    let t113987 = t31176 * t1369;
    let t114000 = t22804 * t31156;
    let t114002 = t3777 * t31169;
    let t114011 = t1336 * t1338 * t835 * t241;
    let t114012 = t114011 * t31172;
    (t113982, t113987, t114000, t114002, t114011, t114012)
}
