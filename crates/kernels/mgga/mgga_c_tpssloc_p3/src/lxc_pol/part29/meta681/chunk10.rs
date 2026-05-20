//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2303/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2303<F: Float>(t27533: F, t86094: F, t24826: F, t27521: F, t1235: F, t1244: F, t1246: F, t1734: F, t24589: F, t24745: F, t24757: F, t24777: F, t24788: F, t24858: F, t27453: F, t27454: F, t27465: F, t27516: F, t27549: F, t27550: F, t3242: F, t3961: F, t7283: F, t8066: F, t85832: F, t86001: F, t94400: F, t94404: F) -> F {
    let t95163 = F::cast_from(0.18277045187202515961e-2_f64) * t86094 * t27533;
    let t95165 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27521;
    let t95184 = -F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t94400 * t27454 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t94404 * t27454 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t27453 * t24745 * t24858 - t95163 + t95165 + F::cast_from(0.73108180748810063846e-2_f64) * t27549 * t27550 * t1235 * t3242 * t3961 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t85832 * t8066 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24788 * t27465 - F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t27516 * t24777 + t1244 * t24757 * t1734 * t1246 - F::cast_from(0.54831135561607547884e-2_f64) * t86001;
    t95184
}
