//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 896/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk896<F: Float>(t31: F, t625: F, t31864: F, t607: F, t8308: F, t63: F, t645: F, t31857: F, t32344: F, t31868: F, t240: F, t8307: F, t8513: F, t8663: F, t113836: F, t113875: F, t116088: F, t116096: F, t116099: F, t116111: F, t116115: F, t116119: F, t2250: F, t2303: F, t32331: F, t32333: F, t32338: F, t641: F, t8825: F) -> (F,) {
    let t117496 = t625 * t31;
    let t117499 = t31864 * t8308 * t117496 * t607;
    let t117503 = t63 * t645;
    let t117516 = t31857 * t32344;
    let t117518 = t31868 * t32344;
    let t117527 = 55.0 / 81.0 * t8663 * t8513 * t8307 * t240;
    let t117528 = -5.0 / 36.0 * t8663 * t8513 * t32338 * t2303 - 5.0 / 72.0 * t116096 * t8825 - 5.0 / 36.0 * t116099 * t8825 - 40.0 / 27.0 * t117499 + 5.0 / 9.0 * t116111 * t32333 + 5.0 / 3.0 * t116115 * t113875 * t117503 * t641 + 5.0 / 9.0 * t116119 * t32333 + 5.0 / 18.0 * t31864 * t8308 * t32331 * t2250 - 5.0 / 72.0 * t116088 * t8825 + 10.0 / 27.0 * t117516 + 10.0 / 27.0 * t117518 - 5.0 / 36.0 * t8663 * t8513 * t113836 * t63 - t117527;
    (t117528,)
}
