//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1570/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1570<F: Float>(t23069: F, t805: F, t243: F, t598: F, t213: F, t6584: F, t6604: F) -> (F, F, F, F, F, F) {
    let t23070 = t23069 * t805;
    let t23071 = F::new(7.0) / F::new(72.0) * t23070;
    let t23075 = t243 * t243;
    let t23076 = F::new(1.0) / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23083 = t6584 * t6604;
    (t23071, t23075, t23076, t23077, t23078, t23083)
}
