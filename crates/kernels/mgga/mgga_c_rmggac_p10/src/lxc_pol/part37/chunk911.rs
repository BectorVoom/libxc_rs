//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 911/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk911<F: Float>(t5148: F, t76077: F, t69453: F, t5259: F, t74959: F, t4669: F, t74963: F, t74815: F, t556: F, t69144: F, t2842: F, t69436: F) -> (F, F, F, F, F, F, F) {
    let t76355 = F::cast_from(0.5987120850931904282e-1_f64) * t5148 * t76077;
    let t76356 = F::cast_from(0.79828278012425390427e-1_f64) * t69453;
    let t76358 = F::cast_from(0.5987120850931904282e-1_f64) * t5259 * t74959;
    let t76360 = F::cast_from(0.8980681276397856423e-1_f64) * t4669 * t74963;
    let t76362 = F::cast_from(0.5987120850931904282e-1_f64) * t5148 * t74815;
    let t76363 = t69144 * t556;
    let t76365 = t69436 * t2842;
    (t76355, t76356, t76358, t76360, t76362, t76363, t76365)
}
