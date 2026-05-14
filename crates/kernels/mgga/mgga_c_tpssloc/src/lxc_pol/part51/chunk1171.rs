//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1171/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1171<F: Float>(t32840: F, t849: F, t112802: F, t32844: F, t1499: F, t30719: F, t8344: F, t1894: F, t4119: F, t59: F, t6591: F, t30714: F, t4240: F, t4250: F, t4191: F, t23270: F, t25038: F, t30622: F, t4255: F) -> (F, F, F, F, F, F, F, F) {
    let t118594 = t32840 * t849;
    let t118596 = t112802 * t32844;
    let t118602 = t1499 * t30719 * t8344;
    let t118606 = t6591 * t1894 * t59 * t4119;
    let t118608 = t30714 * t4240;
    let t118610 = t30714 * t4250;
    let t118612 = t30714 * t4191;
    let t118626 = 0.9869604401089358619e-1 * t25038 * t23270 * t30622 * t4255;
    (t118594, t118596, t118602, t118606, t118608, t118610, t118612, t118626)
}
