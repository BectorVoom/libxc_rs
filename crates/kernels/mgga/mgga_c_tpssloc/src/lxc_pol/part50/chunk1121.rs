//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1121/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1121<F: Float>(t33185: F, t8319: F, t1873: F, t7467: F, t3941: F, t5371: F, t8326: F, t1458: F, t31267: F, t33164: F, t33177: F, t33179: F, t33181: F, t33184: F, t577: F, t8508: F) -> (F, F, F, F, F) {
    let t33187 = F::new(27.0) * t33185 * t8319;
    let t33188 = t1873 * t7467;
    let t33190 = F::new(54.0) * t3941 * t33188;
    let t33191 = t5371 * t8326;
    let t33192 = F::new(0.135e2) * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = F::new(27.0) * t33194;
    let t33196 = F::new(0.45e1) * t33164 * t577 + F::new(0.135e2) * t31267 * t1458 + F::new(27.0) * t33177 + F::new(54.0) * t33179 + F::new(27.0) * t33181 + t33184 + t33187 + t33190 + t33192 + t33195 + t8508;
    (t33188, t33192, t33193, t33195, t33196)
}
