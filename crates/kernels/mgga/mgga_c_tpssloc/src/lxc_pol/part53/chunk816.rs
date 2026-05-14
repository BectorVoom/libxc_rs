//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 816/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk816<F: Float>(t1894: F, t7823: F, t214: F, t1880: F, t6571: F, t7841: F, t6553: F, t31366: F, t7479: F, t6552: F, t7488: F, t225: F, t258: F, t1484: F, t31337: F, t23270: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33383 = t1894 * t7823;
    let t33384 = t214 * t33383;
    let t33385 = t1880 * t33384;
    let t33408 = t6571 * t7841;
    let t33409 = t6553 * t33408;
    let t33410 = t1880 * t33409;
    let t33419 = t31366 * t7479;
    let t33420 = t6552 * t33419;
    let t33422 = t31366 * t7488;
    let t33423 = t1880 * t33422;
    let t33428 = t7823 * t225 * t258;
    let t33429 = t214 * t33428;
    let t33430 = t1880 * t33429;
    let t33447 = t31337 * t1484;
    let t33448 = t23270 * t33447;
    (t33383, t33384, t33385, t33408, t33409, t33410, t33419, t33420, t33422, t33423, t33428, t33429, t33430, t33447, t33448)
}
