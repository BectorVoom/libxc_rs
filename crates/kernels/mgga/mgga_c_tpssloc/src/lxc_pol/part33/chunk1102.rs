//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1102/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1102<F: Float>(t43603: F, t68: F, t3215: F, t1406: F, t9238: F, t111: F, t6470: F, t2239: F, t5385: F, t1597: F, t976: F, t3131: F, t5866: F, t20292: F, t21038: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43604 = t68 * t43603;
    let t43636 = t3215 * t3215;
    let t43637 = 1.0 / t43636;
    let t45844 = t1406 * t9238;
    let t55388 = t6470 * t111;
    let t55921 = t5385 * t2239;
    let t61066 = t976 * t1597;
    let t62840 = t5866 * t3131;
    let t67001 = t20292 * t111;
    let t67305 = t21038 * t225;
    (t43604, t43637, t45844, t55388, t55921, t61066, t62840, t67001, t67305)
}
