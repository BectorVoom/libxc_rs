//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1165/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1165<F: Float>(t32441: F, t3572: F, t32440: F, t3535: F, t1207: F, t3068: F, t32439: F, t1222: F, t32436: F, t3540: F, t8879: F, t2139: F, t24745: F) -> (F, F, F, F, F, F) {
    let t117969 = t32441 * t3572;
    let t117973 = t3535 * t32440;
    let t117977 = t1207 * t32439 * t3068;
    let t118002 = t32436 * t1222;
    let t118005 = t8879 * t3540 / F::cast_from(6912.0_f64);
    let t118006 = t24745 * t2139;
    (t117969, t117973, t117977, t118002, t118005, t118006)
}
