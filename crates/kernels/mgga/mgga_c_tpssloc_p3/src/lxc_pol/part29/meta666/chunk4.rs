//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2223/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2223<F: Float>(t26395: F, t3719: F, t6637: F, t6888: F, t3787: F, t7722: F, t16307: F, t90915: F, t91004: F, t81187: F, t81197: F, t1307: F, t26331: F, t26446: F, t90818: F) -> (F, F, F, F, F, F) {
    let t91025 = t6888 * t6637 * t26395 * t3719;
    let t91029 = t3787 * t7722;
    let t91036 = t91004 * t90915 * t16307;
    let t91043 = F::cast_from(0.25587863262083522346e0_f64) * t81187;
    let t91045 = F::cast_from(0.3289868133696452873e-1_f64) * t81197;
    let t91048 = t26331 * t26446 * t90818 * t1307;
    (t91025, t91029, t91036, t91043, t91045, t91048)
}
