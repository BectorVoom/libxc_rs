//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2288/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2288<F: Float>(t22635: F, t26331: F, t26332: F, t3719: F, t1834: F, t213: F, t225: F, t22633: F, t22637: F, t26333: F, t80650: F, t16470: F, t26224: F, t26225: F, t80689: F, t90539: F, t90542: F, t90547: F, t90550: F, t90551: F, t90556: F) -> F {
    let t90560 = t26331 * t22635 * t26332 * t3719;
    let t90566 = t213 * t1834 * t225;
    let t90568 = t22633 * t90566 * t22637;
    let t90571 = t26331 * t80650 * t26333;
    let t90573 = F::cast_from(0.16449340668482264365e-1_f64) * t90539 + t90542 + F::cast_from(0.19190897446562641759e-1_f64) * t80689 + t90547 - t90550 - F::cast_from(0.52089578783527170489e-1_f64) * t90551 + F::cast_from(0.9869604401089358619e-1_f64) * t90556 + F::cast_from(0.49348022005446793095e-1_f64) * t90560 - F::cast_from(6.0_f64) * t26224 * t26225 * t16470 + F::cast_from(0.3289868133696452873e-1_f64) * t90568 + F::cast_from(0.9869604401089358619e-1_f64) * t90571;
    t90573
}
