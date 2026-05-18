//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1292/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1292<F: Float>(t63977: F, t63990: F, t1395: F, t18770: F, t20447: F, t219: F, t1805: F, t8275: F, t1219: F, t6419: F, t10085: F, t1838: F) -> (F, F, F, F, F, F, F) {
    let t66429 = F::new(35.0) / F::new(144.0) * t63977;
    let t66434 = F::new(7.0) / F::new(12.0) * t63990;
    let t66480 = t18770 * t1395;
    let t66525 = t20447 * t219;
    let t66559 = t8275 * t1805;
    let t66970 = t1219 * t6419;
    let t67006 = t10085 * t1838;
    (t66429, t66434, t66480, t66525, t66559, t66970, t67006)
}
