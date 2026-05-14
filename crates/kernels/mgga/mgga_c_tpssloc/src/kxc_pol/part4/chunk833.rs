//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 833/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk833<F: Float>(t11713: F, t11727: F, t11708: F, t3514: F, t11717: F, t1210: F, t3247: F, t415: F, t121: F, t3584: F, t1229: F, t676: F, t1090: F, t248: F, t1227: F, t486: F) -> (F, F, F, F, F, F, F, F) {
    let t11728 = t11713 * t11727;
    let t11734 = t11708 * t3514;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11778 = 1.0 / t415 / t3247;
    let t11784 = t121 * t3584;
    let t11789 = t676 * t1229;
    let t11791 = t248 * t11789 * t1090;
    let t11792 = t1227 * t11791;
    let t11818 = t676 * t486;
    (t11728, t11734, t11738, t11778, t11784, t11789, t11792, t11818)
}
