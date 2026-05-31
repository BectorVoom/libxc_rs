//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1460/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1460<F: Float>(t113875: F, t116106: F, t116111: F, t116115: F, t116119: F, t117447: F, t117451: F, t117461: F, t119879: F, t119883: F, t119891: F, t119901: F, t122941: F, t122945: F, t122952: F, t122955: F, t122976: F, t122979: F, t124755: F, t124778: F, t124803: F, t124805: F, t124807: F, t1409: F, t31864: F, t32331: F, t32333: F, t34126: F, t3966: F, t641: F, t645: F, t8308: F, t84186: F, t8513: F, t8824: F, t8825: F) -> F {
    let t124814 = F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t116115 * t113875 * t124755 * t641 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t116111 * t34126 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t116119 * t34126 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31864 * t8308 * t84186 * t1409 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31864 * t8308 * t32331 * t3966 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t122941 * t8513 * t8824 * t1409 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t122945 * t32333 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t116115 * t113875 * t124778 * t645 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t122976 * t32333 - F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t122979 * t8308 * t124755 * t645 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t116106 * t117447 * t119879 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t116106 * t117447 * t119883 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t31864 * t117451 * t119891 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t31864 * t117451 * t119901 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t124803 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t124805 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t124807 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t122952 * t8825 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t122955 * t8825 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t117461;
    t124814
}
