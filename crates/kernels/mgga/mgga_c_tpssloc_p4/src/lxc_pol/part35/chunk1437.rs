//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1437/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1437<F: Float>(t103175: F, t103261: F, t103286: F, t103363: F, t103464: F, t109060: F, t1238: F, t1761: F, t21510: F, t2154: F, t22393: F, t24589: F, t24601: F, t27382: F, t27549: F, t27774: F, t27820: F, t29678: F, t29690: F, t29798: F, t29822: F, t3598: F, t4945: F, t6140: F, t7283: F, t7300: F, t7301: F, t8002: F, t8011: F, t8014: F, t85642: F, t94395: F, t94436: F, t94476: F) -> F {
    let t109137 = F::cast_from(0.16449340668482264365e-1_f64) * t103261 - F::cast_from(0.54831135561607547884e-2_f64) * t94436 - F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t103363 * t8014 - F::new(18.0) * t4945 * t29798 + F::cast_from(0.24674011002723396548e-1_f64) * t7283 * t6140 * t27382 + F::cast_from(0.54831135561607547884e-2_f64) * t94476 - F::new(3.0) * t103464 * t1761 + F::cast_from(0.24125699647107321069e0_f64) * t29678 * t8011 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t7300 * t7301 * t22393 - F::cast_from(0.10966227112321509577e-1_f64) * t27549 * t24601 * t85642 * t109060 + F::cast_from(0.10966227112321509577e-1_f64) * t27549 * t24601 * t27774 * t21510 - F::cast_from(0.10966227112321509577e-1_f64) * t27549 * t27820 * t29690 - F::cast_from(0.43864908449286038307e-1_f64) * t94395 * t29822 + F::cast_from(0.82246703342411321826e-2_f64) * t24589 * t103175 * t8002 + F::cast_from(0.43864908449286038307e-1_f64) * t103286 + F::new(2.0) * t1238 * t3598 * t2154 * t22393;
    t109137
}
