//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1565/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1565<F: Float>(t2844: F, t4395: F, t912: F, t2842: F, t2836: F, t4399: F, t10704: F, t1556: F, t2793: F, t10702: F, t13566: F, t13602: F) -> (F, F, F, F, F) {
    let t14388 = t4395 * t2844;
    let t14389 = t14388 * t912;
    let t14391 = F::cast_from(0.32163958997385070134e2_f64) * t2842 * t14389;
    let t14392 = t4399 * t2836;
    let t14394 = F::cast_from(0.16081979498692535067e2_f64) * t2842 * t14392;
    let t14395 = t1556 * t10704;
    let t14396 = t14395 * t2793;
    let t14398 = F::cast_from(0.51726012919273400301e3_f64) * t10702 * t14396;
    let t14409 = F::cast_from(0.2283111111111111111e-1_f64) * t13566;
    let t14410 = F::cast_from(0.11415555555555555555e-1_f64) * t13602;
    (t14391, t14394, t14398, t14409, t14410)
}
