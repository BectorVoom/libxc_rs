//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 961/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk961<F: Float>(t1294: F, t9722: F, t172: F, t3681: F, t763: F, t2528: F, t3691: F, t9919: F, t2663: F, t3814: F, t67: F, t758: F) -> (F, F, F, F, F, F) {
    let t12087 = F::new(0.10389515463408878255e3) * t1294 * t9722;
    let t12088 = t3681 * t172;
    let t12089 = t12088 * t763;
    let t12091 = t3691 * t2528;
    let t12094 = F::new(0.35089341735807877242e1) * t1294 * t9919;
    let t12097 = t3814 * t2663;
    let t12099 = t3681 * t67;
    let t12100 = t12099 * t758;
    (t12087, t12089, t12091, t12094, t12097, t12100)
}
