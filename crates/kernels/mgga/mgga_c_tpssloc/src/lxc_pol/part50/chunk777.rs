//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 777/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk777<F: Float>(t3: F, t8496: F, t1873: F, t7010: F, t3941: F, t8319: F, t1401: F, t8326: F, t577: F, t131: F, t8306: F) -> (F, F, F, F) {
    let t8497 = t3 * t8496;
    let t8503 = t7010 * t1873;
    let t8506 = 27.0 * t3941 * t8319;
    let t8508 = 0.135e2 * t1401 * t8326;
    let t8509 = 0.45e1 * t8496 * t577 + 27.0 * t8503 + t8506 + t8508;
    let t8513 = t131 * t8306;
    (t8497, t8508, t8509, t8513)
}
