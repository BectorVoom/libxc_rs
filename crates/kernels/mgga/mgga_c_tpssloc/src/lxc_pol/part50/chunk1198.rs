//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1198/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1198<F: Float>(t118429: F, t118964: F, t1054: F, t7624: F, t23384: F, t32976: F, t33001: F, t113177: F, t113236: F, t14545: F, t1927: F, t1955: F, t23327: F, t23329: F, t23336: F, t23394: F, t25742: F, t25759: F, t25814: F, t2775: F, t30781: F, t30915: F, t32924: F, t32969: F, t32970: F, t32980: F, t3961: F, t4347: F, t4694: F, t6680: F, t6687: F, t6704: F, t7553: F, t7599: F, t82402: F, t82417: F, t8407: F, t88112: F, t883: F, t884: F, t88772: F) -> (F, F, F) {
    let t118965 = t118429 + t118964;
    let t118971 = t1054 * t7624;
    let t119008 = t23384 * t32976;
    let t119010 = t23384 * t33001;
    let t119016 = -F::new(0.9869604401089358619e-1) * t1927 * t23329 * t25759 + F::new(0.14621636149762012769e-1) * t82402 * t32970 - F::new(0.54831135561607547883e-2) * t23327 * t23329 * t118971 * t884 - F::new(0.54831135561607547883e-2) * t23327 * t23329 * t30781 * t4347 - F::new(0.54831135561607547883e-2) * t23327 * t113236 * t7553 + F::new(0.10966227112321509577e-1) * t23327 * t88772 * t7599 * t884 + F::new(0.10966227112321509577e-1) * t23327 * t88112 * t1955 * t2775 * t3961 + F::new(0.10966227112321509577e-1) * t23327 * t88772 * t1955 * t883 * t25814 - F::new(0.3289868133696452873e-1) * t1927 * t23336 * t32980 + F::new(0.54831135561607547883e-2) * t113177 - F::new(0.43864908449286038307e-1) * t6680 * t32924 - t14545 * t8407 + F::new(0.3289868133696452873e-1) * t6687 * t6704 * t23394 * t25742 - F::new(0.54831135561607547883e-2) * t119008 - F::new(0.54831135561607547883e-2) * t119010 - t30915 * t4694 - F::new(0.54831135561607547883e-2) * t23327 * t82417 * t32969;
    (t118965, t118971, t119016)
}
