//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1199/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1199<F: Float>(t32726: F, t539: F, t31137: F, t7691: F, t6888: F, t1375: F, t1843: F, t2016: F, t26477: F, t31106: F, t31113: F, t31189: F, t32686: F, t32690: F, t32696: F, t32700: F, t32707: F, t32708: F, t568: F, t6958: F, t7750: F) -> (F, F, F) {
    let t32727 = t539 * t32726;
    let t32731 = t31137 * t7691;
    let t32733 = F::new(0.3289868133696452873e-1) * t6888 * t32731;
    let t32734 = F::new(2.0) * t1375 * t32686 - F::new(6.0) * t1375 * t32690 - t1843 * t31189 - F::new(2.0) * t2016 * t26477 + t32708 * t568 + t32727 * t568 - F::new(2.0) * t6958 * t7750 - t31106 - t31113 + t32696 - t32700 + t32707 - t32733;
    (t32727, t32731, t32734)
}
