//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1108/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1108<F: Float>(t120063: F, t120064: F, t120067: F, t120069: F, t120072: F, t120075: F, t120078: F, t120079: F, t120083: F, t120085: F, t120086: F, t120088: F, t120092: F, t120095: F, t120097: F, t31055: F, t31057: F, t31060: F) -> (F,) {
    let t120098 = -t31055 - t31057 - t31060 - t120063 - 4.0 * t120064 - t120067 - t120069 + 2.0 * t120072 - t120075 + t120078 + 6.0 * t120079 - t120083 + t120085 + 6.0 * t120086 + 6.0 * t120088 - t120092 + t120095 - t120097;
    (t120098,)
}
