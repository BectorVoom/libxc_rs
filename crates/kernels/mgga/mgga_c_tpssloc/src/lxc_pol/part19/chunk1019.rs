//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1019/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1019<F: Float>(t12289: F, t242: F, t1336: F, t3804: F, t820: F, t3788: F, t836: F, t3777: F, t5245: F, t3734: F, t571: F, t2319: F, t576: F, t1351: F, t1372: F, t154: F, t2558: F) -> (F, F, F, F, F, F, F, F) {
    let t16232 = t12289 * t242;
    let t16233 = t1336 * t16232;
    let t16305 = t3804 * t820;
    let t16397 = t3788 * t836;
    let t16398 = t1336 * t16397;
    let t16401 = t3777 * t5245;
    let t16490 = t3734 * t571;
    let t16535 = t576 * t2319;
    let t22694 = t1372 * t1351;
    let t22715 = t2558 * t154;
    (t16233, t16305, t16398, t16401, t16490, t16535, t22694, t22715)
}
