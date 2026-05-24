//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1073/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1073<F: Float>(t4601: F, t9008: F, t27036: F, t681: F, t26346: F, t7710: F, t117: F, t29933: F, t2295: F, t40906: F, t8640: F, t2038: F, t39116: F, t7756: F, t7933: F) -> (F, F, F, F, F, F) {
    let t42151 = t4601 * t9008;
    let t42152 = F::cast_from(0.23948483403727617128e0_f64) * t42151;
    let t42156 = t27036 * t681;
    let t42159 = t26346 * t7710;
    let t42161 = t29933 * t117;
    let t42162 = t42161 * t2295;
    let t42166 = t8640 * t40906;
    let t42167 = F::cast_from(0.10909864661698136691e0_f64) * t42166;
    let t42170 = t7933 * t2038 * t39116 * t7756;
    (t42152, t42156, t42159, t42162, t42167, t42170)
}
