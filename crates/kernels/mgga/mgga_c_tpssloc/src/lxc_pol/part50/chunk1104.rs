//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1104/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1104<F: Float>(t32961: F, t383: F, t1058: F, t1610: F, t1920: F, t30876: F, t32928: F, t32931: F, t32935: F, t32939: F, t32944: F, t353: F, t6687: F, t6797: F, t8404: F) -> (F, F) {
    let t32962 = t383 * t32961;
    let t32964 = t30876 + F::cast_from(0.54831135561607547883e-2_f64) * t6687 * t32928 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t32931 + F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t32935 + F::cast_from(0.16449340668482264365e-1_f64) * t1920 * t32939 + t1610 * t8404 + t1058 * t32944 + t353 * t32962;
    (t32962, t32964)
}
