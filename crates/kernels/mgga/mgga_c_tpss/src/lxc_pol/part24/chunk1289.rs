//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1289/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1289<F: Float>(t21236: F, t5522: F, t13133: F, t6113: F, t13554: F, t1600: F, t19596: F, t626: F, t19574: F, t6243: F, t19577: F, t6275: F, t19585: F, t21018: F, t5706: F, t1163: F, t1338: F, t13458: F, t1753: F, t19315: F, t20078: F, t2056: F, t21171: F, t21241: F, t21532: F, t24587: F, t3493: F, t3499: F, t3538: F, t4638: F, t485: F, t5463: F, t5692: F, t5702: F, t624: F, t69365: F) -> (F,) {
    let t69388 = 2.0 * t21236 * t5522;
    let t69392 = 4.0 * t13133 * t6113;
    let t69394 = 4.0 * t13554 * t6113;
    let t69397 = 4.0 * t626 * t1600 * t19596;
    let t69401 = 2.0 * t6243 * t19574;
    let t69403 = 2.0 * t19577 * t6275;
    let t69420 = 2.0 * t6243 * t19585;
    let t69422 = 6.0 * t5706 * t21018;
    let t69423 = -4.0 * t1338 * t20078 * t626 - t1163 * t21171 - 2.0 * t13458 * t1753 - 4.0 * t19315 * t3493 - 4.0 * t2056 * t21241 - 4.0 * t21241 * t3499 - t21532 * t624 - 4.0 * t24587 * t3538 - 2.0 * t4638 * t5692 - t485 * t69365 + t5463 * t5702 - t69388 - t69392 - t69394 - t69397 - t69401 + t69403 - t69420 + t69422;
    (t69423,)
}
