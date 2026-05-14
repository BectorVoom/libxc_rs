//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1098/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1098<F: Float>(t1232: F, t1265: F, t520: F, t1258: F, t3255: F, t1270: F, t3245: F, t196: F, t197: F, t3174: F, t508: F, t1759: F) -> (F, F, F, F, F, F) {
    let t18499 = t1265 * t1232 * t520;
    let t18511 = t1258 * t3255;
    let t18539 = t1270 * t3245;
    let t18544 = t3174 * t196 * t197;
    let t18546 = t197 * t508;
    let t18547 = t1759 * t18546;
    (t18499, t18511, t18539, t18544, t18546, t18547)
}
