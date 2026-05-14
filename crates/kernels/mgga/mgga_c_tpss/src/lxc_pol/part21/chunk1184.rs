//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1184/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1184<F: Float>(t18403: F, t485: F, t626: F, t1163: F, t5531: F, t1683: F, t2061: F, t1688: F, t7798: F, t10456: F, t2056: F, t13146: F, t4347: F, t1165: F, t17916: F, t18375: F, t2105: F, t5514: F, t645: F) -> (F, F, F, F, F, F) {
    let t18404 = t485 * t18403;
    let t18406 = 2.0 * t626 * t18404;
    let t18409 = t1163 * t5531;
    let t18411 = 4.0 * t626 * t18409;
    let t18414 = t1683 * t2061;
    let t18419 = 2.0 * t7798 * t1688;
    let t18421 = 4.0 * t10456 * t1688;
    let t18423 = 4.0 * t2056 * t5531;
    let t18425 = 2.0 * t13146 * t1688;
    let t18427 = 4.0 * t4347 * t5531;
    let t18429 = 2.0 * t1165 * t18403;
    let t18430 = 4.0 * t17916 * t645 + 2.0 * t2105 * t5514 + t18375 + 2.0 * t18414 + t18419 + t18421 + t18423 + t18425 + t18427 + t18429;
    (t18404, t18406, t18409, t18411, t18414, t18430)
}
