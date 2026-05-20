//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1278/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1278<F: Float>(t1193: F, t22104: F, t22038: F, t3448: F, t20234: F, t44607: F, t15376: F, t18446: F, t15338: F, t18427: F, t3447: F, t22032: F) -> (F, F, F, F, F, F) {
    let t73142 = t22104 * t1193;
    let t73169 = t3448 * t22038;
    let t73181 = t44607 * t20234;
    let t73188 = t15376 * t18446;
    let t73199 = t3447 * t15338 * t18427;
    let t73201 = t3448 * t22032;
    (t73142, t73169, t73181, t73188, t73199, t73201)
}
