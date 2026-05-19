//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 224/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk224<F: Float>(t745: F, t746: F, t118: F, t168: F, t181: F, t677: F, t680: F, t705: F, t725: F, t732: F, t740: F, t157: F) -> (F, F, F) {
    let t747 = t745 * t746;
    let t750 = F::cast_from(0.53237641966666666666e-3_f64) * t118 * t677 * t168 + F::new(1.0) * t725 * t732 - t680 - t705 + F::cast_from(0.18311447306006545054e-3_f64) * t118 * t677 * t181 + F::cast_from(0.5848223622634646207e0_f64) * t740 * t747;
    let t751 = t157 * t750;
    (t747, t750, t751)
}
