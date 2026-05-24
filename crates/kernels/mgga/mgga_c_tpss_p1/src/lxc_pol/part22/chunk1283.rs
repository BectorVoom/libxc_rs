//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1283/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1283<F: Float>(t3205: F, t5935: F, t36: F, t68: F, t581: F, t6435: F, t1270: F, t3204: F, t10178: F, t536: F, t1974: F, t1980: F) -> (F, F, F, F, F, F, F) {
    let t24128 = t3205 * t5935;
    let t24289 = t68 * t36;
    let t24290 = t24289 * t581;
    let t25469 = t3205 * t6435;
    let t26207 = t5935 * t1270;
    let t30366 = t3204 * t3204;
    let t30367 = F::new(1.0) / t30366;
    let t31297 = F::new(1.0) / t10178 / t536;
    let t31450 = t1974 * t1980;
    (t24128, t24290, t25469, t26207, t30367, t31297, t31450)
}
