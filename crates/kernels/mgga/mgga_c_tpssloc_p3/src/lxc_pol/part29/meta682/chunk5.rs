//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2309/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2309<F: Float>(t15743: F, t7345: F, t24649: F, t27710: F, t23508: F, t8026: F, t7325: F, t27628: F, t7324: F, t7331: F, t15730: F, t7339: F) -> (F, F, F, F, F) {
    let t95320 = F::new(5.0) / F::new(5184.0) * t7345 * t15743;
    let t95323 = t27710 * t24649;
    let t95326 = t8026 * t23508;
    let t95327 = t95326 * t7325;
    let t95332 = t7324 * t27628;
    let t95334 = F::cast_from(0.20186378047070195428e-3_f64) * t95332 * t7331;
    let t95335 = t7339 * t15730;
    (t95320, t95323, t95327, t95334, t95335)
}
