//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1068/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1068<F: Float>(t27075: F, t739: F, t7577: F, t35523: F, t9222: F, t36733: F, t8450: F, t7478: F, t1970: F, t209: F, t236: F, t40433: F, t7231: F) -> (F, F, F, F) {
    let t42081 = t739 * t7577 * t27075;
    let t42083 = t9222 * t35523;
    let t42085 = t8450 * t36733;
    let t42086 = t42085 * t7478;
    let t42087 = F::cast_from(0.19863479950205658386e-4_f64) * t42086;
    let t42091 = t1970 * t7231 * t236 * t40433 * t209;
    (t42081, t42083, t42087, t42091)
}
