//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2095/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2095<F: Float>(t24574: F, t27779: F, t8015: F, t85660: F, t27826: F, t27403: F, t27389: F, t8074: F, t85917: F, t24826: F, t27511: F, t15394: F, t2127: F, t221: F) -> (F, F, F, F, F, F, F, F) {
    let t94700 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27779;
    let t94701 = t85660 * t8015;
    let t94710 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27826;
    let t94759 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27403;
    let t94779 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27389;
    let t94784 = t85917 * t8074;
    let t94787 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27511;
    let t94796 = t2127 * t221 * t15394;
    (t94700, t94701, t94710, t94759, t94779, t94784, t94787, t94796)
}
