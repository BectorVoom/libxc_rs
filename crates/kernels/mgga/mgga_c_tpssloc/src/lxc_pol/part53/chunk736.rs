//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 736/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk736<F: Float>(t6883: F, t7697: F, t22897: F, t5336: F, t1992: F, t22751: F, t7733: F, t1799: F, t22881: F, t6637: F, t6888: F, t5187: F, t6968: F, t22893: F, t7732: F, t22892: F) -> (F, F, F, F, F, F) {
    let t26361 = t6883 * t7697;
    let t26378 = t22897 * t5336;
    let t26379 = t1992 * t26378;
    let t26381 = t22751 * t7733;
    let t26384 = t22881 * t1799;
    let t26385 = t6637 * t26384;
    let t26386 = t6888 * t26385;
    let t26388 = t6968 * t5187;
    let t26389 = t6637 * t26388;
    let t26390 = t6888 * t26389;
    let t26392 = t22893 * t7732;
    let t26393 = t22892 * t26392;
    (t26361, t26379, t26381, t26386, t26390, t26393)
}
