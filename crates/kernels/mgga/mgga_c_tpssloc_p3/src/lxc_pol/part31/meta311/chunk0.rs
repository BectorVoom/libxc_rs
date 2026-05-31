//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1200/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1200<F: Float>(t3242: F, t460: F, t3247: F, t1176: F, t134: F, t1184: F, t1239: F, t68: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F) -> (F, F, F, F, F, F, F) {
    let t11570 = t460 * t3242;
    let t11583 = t460 * t3247;
    let t11588 = t134 * t1176;
    let t11589 = t11588 * t1184;
    let t11604 = t1239 * t1239;
    let t11605 = F::cast_from(1.0_f64) / t11604;
    let t11606 = t68 * t11605;
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    (t11570, t11583, t11588, t11589, t11606, t11644, t11647)
}
