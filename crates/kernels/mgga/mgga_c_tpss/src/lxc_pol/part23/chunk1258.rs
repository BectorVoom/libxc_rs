//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1258/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1258<F: Float>(t18351: F, t5975: F, t1860: F, t61938: F, t116: F, t19239: F, t1906: F, t3398: F, t19279: F, t550: F, t19300: F, t546: F, t1284: F, t6061: F, t1901: F, t3413: F) -> (F, F, F, F, F, F, F, F) {
    let t63597 = t5975 * t18351;
    let t63600 = t1860 * t61938;
    let t63626 = t19239 * t116;
    let t63662 = t3398 * t1906;
    let t63664 = t19279 * t550;
    let t63667 = t546 * t19300;
    let t63669 = t6061 * t1284;
    let t63675 = t1901 * t3413;
    (t63597, t63600, t63626, t63662, t63664, t63667, t63669, t63675)
}
