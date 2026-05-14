//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 799/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk799<F: Float>(t578: F, t582: F, t599: F, t615: F, t76: F, t1689: F, t2056: F, t3499: F, t1163: F, t1688: F) -> (F, F, F, F, F, F) {
    let t5492 = t578 * t582;
    let t5500 = 8.0 / 3.0 * t599;
    let t5506 = t76 * t615;
    let t5519 = 2.0 * t2056 * t1689;
    let t5521 = 2.0 * t3499 * t1689;
    let t5522 = t1163 * t1688;
    (t5492, t5500, t5506, t5519, t5521, t5522)
}
