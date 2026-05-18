//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 218/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk218<F: Float>(t1191: F, t1195: F, t1227: F, t1229: F, t1467: F, t1470: F, t1473: F, t1477: F, t1497: F, t1500: F, t1503: F, t1510: F, t1513: F, t1518: F, t1522: F, t467: F, t488: F) -> F {
    let t1525 = F::new(0.54879112805223954488e-1) * t1467 * t1470 + F::new(0.64025631606094613569e-1) * t1473 + F::new(0.54879112805223954488e-1) * t1195 * t1477 - F::new(0.27439556402611977244e-1) * t467 * t1497 - F::new(0.27439556402611977244e-1) * t1500 * t1503 + F::new(0.64025631606094613569e-1) * t1191 + t1227 + F::new(0.12805126321218922714e0) * t1229 + F::new(0.54879112805223954488e-1) * t1195 * t1510 + F::new(0.12805126321218922714e0) * t1513 + F::new(0.16463733841567186346e0) * t488 * t1518 - F::new(0.54879112805223954488e-1) * t488 * t1522;
    t1525
}
