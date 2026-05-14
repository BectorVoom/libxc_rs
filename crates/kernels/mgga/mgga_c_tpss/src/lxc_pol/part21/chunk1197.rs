//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1197/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1197<F: Float>(t18377: F, t18561: F, t3: F, t1786: F, t3403: F, t1279: F, t5773: F, t5776: F, t1688: F, t2061: F, t547: F, t116: F, t5531: F, t645: F, t2105: F, t5772: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18562 = t18377 + t18561;
    let t18563 = t3 * t18562;
    let t18575 = param_d * t18562;
    let t18584 = 3.0 * t3403 * t1786;
    let t18586 = 12.0 * t1279 * t5773;
    let t18588 = 6.0 * t1279 * t5776;
    let t18589 = t2061 * t1688;
    let t18591 = 6.0 * t547 * t18589;
    let t18592 = t116 * t5531;
    let t18593 = t18592 * t645;
    let t18595 = 12.0 * t547 * t18593;
    let t18596 = t5772 * t2105;
    (t18562, t18563, t18575, t18584, t18586, t18588, t18589, t18591, t18592, t18593, t18595, t18596)
}
