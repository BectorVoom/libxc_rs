//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1707/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1707<F: Float>(t3793: F, t3805: F, t5301: F, t3802: F, t5234: F, t3788: F, t836: F, t1336: F, t5252: F, t3777: F, t5245: F, t12419: F, t12420: F, t5249: F) -> (F, F, F, F, F) {
    let t16391 = t3805 * t5301 * t3793;
    let t16394 = t5234 * t3802;
    let t16397 = t3788 * t836;
    let t16398 = t1336 * t16397;
    let t16400 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t16398 * t5252;
    let t16401 = t3777 * t5245;
    let t16405 = t12419 * t5249 * t12420;
    (t16391, t16394, t16400, t16401, t16405)
}
