//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 975/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk975<F: Float>(t1971: F, t236: F, t5624: F, t7365: F, t36332: F, t36334: F, t36344: F, t36379: F, t36381: F, t36383: F, t4041: F, t40630: F, t40637: F, t40647: F, t40652: F, t40655: F, t40659: F, t40662: F, t40664: F, t40668: F, t4965: F, t8387: F, t8390: F) -> F {
    let t40672 = t7365 * t1971 * t236 * t5624;
    let t40674 = F::cast_from(0.44903406381989282115e-1_f64) * t40630 - F::cast_from(0.4726e1_f64) * t36332 - F::cast_from(0.2363e1_f64) * t36334 + F::cast_from(0.51077519871957407277e-4_f64) * t40637 - F::cast_from(0.24829349937757072982e-4_f64) * t36344 - F::cast_from(0.59590439850616975158e-4_f64) * t36379 - F::cast_from(0.23948483403727617128e0_f64) * t4041 * t8387 - F::cast_from(0.23948483403727617128e0_f64) * t4965 * t8390 - F::cast_from(0.19863479950205658386e-4_f64) * t36381 - F::cast_from(0.19863479950205658386e-4_f64) * t36383 - F::cast_from(0.13637330827122670864e-1_f64) * t40647 + F::cast_from(0.12769379967989351819e-4_f64) * t40652 - t40655 - F::cast_from(0.31923449919973379548e-4_f64) * t40659 - F::cast_from(0.85129199786595678796e-5_f64) * t40662 + F::cast_from(0.17025839957319135759e-4_f64) * t40664 + F::cast_from(0.17025839957319135759e-4_f64) * t40668 + F::cast_from(0.85129199786595678796e-5_f64) * t40672;
    t40674
}
