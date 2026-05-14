//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 769/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk769<F: Float>(t32837: F, t6605: F, t1499: F, t8342: F, t8344: F, t232: F, t4180: F, t4181: F, t30714: F, t1516: F, t8343: F, t1527: F, t30633: F, t23270: F, t1888: F, t6571: F, t7537: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32838 = t6605 * t32837;
    let t32840 = t1499 * t8342;
    let t32841 = t32840 * t8344;
    let t32844 = t4180 * t4181 * t232;
    let t32845 = t30714 * t32844;
    let t32847 = t8343 * t1516;
    let t32862 = t30633 * t1527;
    let t32863 = t23270 * t32862;
    let t32865 = 0.3289868133696452873e-1 * t1888 * t32863;
    let t32866 = t6571 * t7537;
    (t32838, t32840, t32841, t32844, t32845, t32847, t32862, t32863, t32865, t32866)
}
