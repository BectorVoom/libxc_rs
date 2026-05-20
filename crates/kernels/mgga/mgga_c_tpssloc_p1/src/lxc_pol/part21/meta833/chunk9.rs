//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2950/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2950<F: Float>(t13822: F, t17757: F, t973: F, t17772: F, t2970: F, t13931: F, t17773: F, t17841: F, t2960: F, t343: F, t4546: F, t48292: F, t48297: F, t48302: F, t48317: F, t48320: F, t48328: F, t55677: F, t7577: F, t977: F, t978: F, t984: F) -> F {
    let t61427 = t973 * t13822 * t17757;
    let t61447 = t973 * t2970 * t17772;
    let t61453 = -F::cast_from(0.55555555555555555554e-3_f64) * t61427 - F::cast_from(0.16666666666666666666e-2_f64) * t973 * t4546 * t17841 * t984 * t343 + F::cast_from(0.74074074074074074072e-3_f64) * t48292 + F::cast_from(0.29629629629629629628e-2_f64) * t48297 + F::cast_from(0.18518518518518518518e-3_f64) * t48302 - F::cast_from(0.98765432098765432096e-3_f64) * t48317 - F::cast_from(0.24691358024691358024e-3_f64) * t48320 + F::cast_from(0.6584362139917695473e-3_f64) * t48328 - F::cast_from(0.16666666666666666666e-2_f64) * t973 * t4546 * t7577 * t13931 - F::cast_from(0.14814814814814814814e-2_f64) * t2960 * t17773 + F::cast_from(0.18518518518518518518e-3_f64) * t61447 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t977 * t978 * t55677;
    t61453
}
