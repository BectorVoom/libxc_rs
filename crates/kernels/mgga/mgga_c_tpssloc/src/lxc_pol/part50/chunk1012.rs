//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1012/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1012<F: Float>(t33191: F, t1458: F, t8326: F, t3941: F, t31267: F, t33164: F, t33177: F, t33179: F, t33181: F, t33184: F, t33187: F, t33190: F, t577: F, t8508: F, t1441: F, t1873: F) -> (F, F, F, F, F) {
    let t33192 = 0.135e2 * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = 27.0 * t33194;
    let t33196 = 0.45e1 * t33164 * t577 + 0.135e2 * t31267 * t1458 + 27.0 * t33177 + 54.0 * t33179 + 27.0 * t33181 + t33184 + t33187 + t33190 + t33192 + t33195 + t8508;
    let t33211 = t1441 * t1873;
    (t33192, t33193, t33195, t33196, t33211)
}
