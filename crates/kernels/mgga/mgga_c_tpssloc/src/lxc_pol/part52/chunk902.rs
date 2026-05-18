//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 902/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk902<F: Float>(t3: F, t8692: F, t1873: F, t7423: F, t577: F, t8503: F, t8506: F, t8508: F, t192: F, t533: F, t1390: F, t2018: F) -> (F, F, F, F) {
    let t8693 = t3 * t8692;
    let t8699 = t7423 * t1873;
    let t8702 = F::new(0.45e1) * t8692 * t577 + F::new(0.135e2) * t8699 + F::new(0.135e2) * t8503 + t8506 + t8508;
    let t8944 = t192 * t533;
    let t8945 = t2018 * t1390;
    (t8693, t8702, t8944, t8945)
}
