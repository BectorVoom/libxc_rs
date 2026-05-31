//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2086/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2086<F: Float>(t1012: F, t10515: F, t6753: F, t1933: F, t23479: F, t82916: F, t23413: F, t344: F, t6740: F, t1016: F, t3034: F, t1930: F) -> (F, F, F, F) {
    let t82964 = t1012 * t6753 * t10515;
    let t82971 = t1933 * t82916 * t23479;
    let t82981 = t6740 * t23413 * t344;
    let t82985 = F::cast_from(1.0_f64) / t3034 / t1016;
    let t82986 = t1930 * t82985;
    (t82964, t82971, t82981, t82986)
}
