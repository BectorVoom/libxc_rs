//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 454/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk454<F: Float>(t8716: F, t8718: F, t8125: F, t8702: F, t8706: F, t8714: F, t8720: F, t8722: F, t8724: F, t8726: F, t9445: F, t8735: F) -> (F, F) {
    let t9447 = F::cast_from(0.18183107769496894486e-1_f64) * t8716;
    let t9448 = F::cast_from(0.24244143692662525982e-1_f64) * t8718;
    let t9453 = -F::cast_from(0.90915538847484472432e-2_f64) * t8702 + F::cast_from(0.1814407727691612783e-3_f64) * t8706 - t9445 + F::cast_from(0.56448240417072397693e-3_f64) * t8714 - t9447 + t9448 - F::cast_from(0.21168090156402149135e-3_f64) * t8720 + F::cast_from(0.68186654135613354324e-2_f64) * t8722 + F::cast_from(0.39828462315181744017e-2_f64) * t8724 - F::cast_from(0.55759847241254441624e-2_f64) * t8726 + t8125;
    let t9457 = F::cast_from(0.17701538806747441785e-2_f64) * t8735;
    (t9453, t9457)
}
