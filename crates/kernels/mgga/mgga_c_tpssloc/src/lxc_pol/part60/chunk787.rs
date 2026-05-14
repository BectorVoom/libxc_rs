//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 787/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk787<F: Float>(t32834: F, t6591: F, t1510: F, t6612: F, t6605: F, t1499: F, t8342: F, t8344: F, t232: F, t4180: F, t4181: F, t30714: F, t1516: F, t8343: F, t1527: F, t30633: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32835 = t6591 * t32834;
    let t32837 = t6612 * t1510;
    let t32838 = t6605 * t32837;
    let t32840 = t1499 * t8342;
    let t32841 = t32840 * t8344;
    let t32844 = t4180 * t4181 * t232;
    let t32845 = t30714 * t32844;
    let t32847 = t8343 * t1516;
    let t32862 = t30633 * t1527;
    (t32835, t32837, t32838, t32840, t32841, t32844, t32845, t32847, t32862)
}
