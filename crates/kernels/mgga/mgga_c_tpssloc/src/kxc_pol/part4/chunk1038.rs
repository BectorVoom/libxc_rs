//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1038/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1038<F: Float>(t18630: F, t18673: F, t18789: F, t18906: F, t300: F, t3400: F, t6084: F, t4883: F, t1164: F, t18247: F, t18249: F, t18251: F, t18257: F, t18261: F, t18264: F, t18268: F, t18270: F, t18273: F, t18278: F, t18282: F, t18285: F, t18672: F, t18676: F, t18679: F) -> (F, F, F) {
    let t18909 = t300 * (t18630 + t18673 + t18789 + t18906);
    let t18910 = t3400 * t6084;
    let t18911 = t18910 * t4883;
    let t18913 = 0.17315859105681463759e2 * t1164 * t18911;
    let t18914 = -t18247 - t18249 - t18251 - t18257 + t18261 + t18264 + t18268 - t18270 - t18273 - t18278 + t18282 - t18285 + t18909 - t18913 - t18672 + t18676 + t18679;
    (t18909, t18913, t18914)
}
