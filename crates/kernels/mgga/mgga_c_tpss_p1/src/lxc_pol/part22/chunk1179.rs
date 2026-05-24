//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1179/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1179<F: Float>(t4341: F, t645: F, t13220: F, t485: F, t2105: F, t94: F, t1600: F, t1168: F, t13131: F, t13133: F, t13136: F, t13223: F, t1339: F, t1663: F, t2056: F, t2106: F, t3174: F, t3493: F, t3499: F, t3502: F, t3538: F, t3542: F, t4541: F, t488: F, t544: F, t626: F, t646: F) -> (F, F, F, F, F) {
    let t13225 = t4341 * t645;
    let t13228 = t485 * t13220;
    let t13235 = t94 * t2105;
    let t13244 = t1600 * t2105;
    let t13251 = F::new(2.0) * t1168 * t4541 + t13131 * t488 - F::new(4.0) * t13133 * t646 - F::new(2.0) * t13136 * t485 + t13223 * t544 - F::new(4.0) * t13225 * t626 - F::new(2.0) * t13228 * t626 - F::new(2.0) * t13235 * t1339 - F::new(2.0) * t13244 * t626 + t1663 * t3174 - F::new(4.0) * t2056 * t3538 - F::new(4.0) * t2056 * t3542 - F::new(2.0) * t2106 * t3493 - F::new(4.0) * t3499 * t3502 - F::new(4.0) * t3499 * t3538;
    (t13225, t13228, t13235, t13244, t13251)
}
