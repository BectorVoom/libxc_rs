//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1166/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1166<F: Float>(t19407: F, t77: F, t1290: F, t1976: F, t3426: F, t578: F, t3432: F, t1600: F, t5531: F, t626: F, t2056: F, t6113: F, t3499: F, t1163: F, t6112: F, t5706: F, t6275: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19408 = t77 * t19407;
    let t19411 = t1976 * t1290;
    let t19414 = t578 * t3426;
    let t19417 = t578 * t3432;
    let t19434 = t1600 * t5531;
    let t19436 = 2.0 * t626 * t19434;
    let t19438 = 2.0 * t2056 * t6113;
    let t19440 = 2.0 * t3499 * t6113;
    let t19441 = t1163 * t6112;
    let t19443 = 2.0 * t626 * t19441;
    let t19452 = t5706 * t6275;
    (t19408, t19411, t19414, t19417, t19434, t19436, t19438, t19440, t19441, t19443, t19452)
}
