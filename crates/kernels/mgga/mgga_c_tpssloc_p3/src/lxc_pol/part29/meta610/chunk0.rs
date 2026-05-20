//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2049/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2049<F: Float>(t645: F, t6509: F, t1864: F, t2307: F, t2240: F, t2251: F, t835: F, t22573: F, t6875: F, t22947: F, t532: F, t2169: F, t3946: F) -> (F, F, F, F, F, F, F) {
    let t83728 = t6509 * t645;
    let t83737 = t1864 * t2307;
    let t83778 = t2240 * t2251;
    let t83803 = F::new(1232.0) / F::new(27.0) * t835;
    let t83886 = t6875 * t22573;
    let t83929 = t532 * t22947;
    let t85403 = t2169 * t3946;
    (t83728, t83737, t83778, t83803, t83886, t83929, t85403)
}
