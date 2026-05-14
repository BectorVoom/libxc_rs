//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 326/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk326<F: Float>(t5: F, t2098: F, t3: F, t1401: F, t2039: F, t577: F, t50: F, t56: F, t63: F, t67: F, t1864: F, t1860: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t2099 = t3 * t2098;
    let t2105 = 0.45e1 * t2098 * t577 + 0.135e2 * t1401 * t2039;
    let t2108 = t50 * t56 - t63;
    let t2109 = t2108 * t67;
    let t2110 = t2109 * t1864;
    let t2113 = piecewise3(t8, 0.0, -t1860 * t2110 / 6.0);
    (t2099, t2105, t2108, t2109, t2110, t2113)
}
