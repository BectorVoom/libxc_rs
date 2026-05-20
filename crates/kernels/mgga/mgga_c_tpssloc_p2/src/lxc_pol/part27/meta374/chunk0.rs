//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1542/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1542<F: Float>(t13969: F, t4599: F, t3039: F, t376: F, t4649: F, t4594: F, t4582: F, t3120: F, t3131: F, t4593: F, t10482: F, t3040: F) -> (F, F, F, F, F, F, F) {
    let t13970 = t13969 * t4599;
    let t13972 = t3039 * t13970 / F::new(2304.0);
    let t13975 = t376 * t4649;
    let t13976 = t13975 * t4594;
    let t13977 = t4582 * t13976;
    let t13980 = t3131 * t3120;
    let t13981 = t4593 * t13980;
    let t13982 = t4582 * t13981;
    let t13985 = t10482 * t3040;
    (t13970, t13972, t13975, t13977, t13980, t13982, t13985)
}
