//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 972/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk972<F: Float>(t2569: F, t3848: F, t1436: F, t8890: F, t2551: F, t3883: F, t903: F, t1449: F, t2613: F, t2595: F, t3886: F, t2621: F, t3882: F, t1448: F, t8752: F, t11240: F, t11242: F, t2575: F, t2594: F, t2619: F, t3865: F, t3887: F, t8888: F, t8906: F, t8912: F, t8915: F, t8922: F) -> (F,) {
    let t11379 = t3848 * t2569;
    let t11382 = t1436 * t8890;
    let t11383 = t11382 * t2551;
    let t11390 = t3883 * t903;
    let t11393 = t1449 * t2613;
    let t11396 = t3886 * t2595;
    let t11399 = t3882 * t2621;
    let t11400 = t11399 * t903;
    let t11403 = t3886 * t2613;
    let t11406 = t1448 * t8752;
    let t11407 = t11406 * t2595;
    let t11410 = t1449 * t2595;
    let t11413 = 0.32163958997385070134e2 * t2575 * t11379 + 0.2069040516770936012e4 * t8888 * t11383 - 0.23392894490538584828e1 * t8906 * t3865 + 0.34631718211362927518e2 * t8912 * t3887 - 0.23392894490538584828e1 * t2594 * t11390 - 0.11696447245269292414e1 * t2594 * t11393 - 0.10389515463408878255e3 * t8915 * t11396 + 0.34631718211362927518e2 * t2619 * t11400 + 0.17315859105681463759e2 * t2619 * t11403 + 0.10254018858216406658e4 * t8922 * t11407 + 0.35089341735807877242e1 * t2619 * t11410 + t11240 - t11242;
    (t11413,)
}
