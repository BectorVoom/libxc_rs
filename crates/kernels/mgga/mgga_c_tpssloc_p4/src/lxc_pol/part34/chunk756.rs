//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 756/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk756<F: Float>(t11713: F, t11727: F, t11717: F, t1210: F, t11153: F, t3439: F, t11147: F, t11545: F, t3247: F, t415: F, t61: F, t121: F, t3584: F) -> (F, F, F, F, F, F) {
    let t11728 = t11713 * t11727;
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11759 = t3439 * t11153;
    let t11764 = t11545 * t11147;
    let t11778 = F::cast_from(1.0_f64) / t415 / t3247;
    let t11779 = t61 * t11778;
    let t11784 = t121 * t3584;
    (t11728, t11738, t11759, t11764, t11779, t11784)
}
