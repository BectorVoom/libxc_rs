//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 883/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk883<F: Float>(t1266: F, t31055: F, t31057: F, t31060: F, t31671: F, t31733: F, t31737: F, t31746: F, t31749: F, t31750: F, t32349: F, t510: F, t7171: F, t7220: F, t8329: F, t8690: F, t8829: F) -> F {
    let t32378 = -t1266 * t8829 - t32349 * t510 + F::new(3.0) * t7171 * t8690 - t7220 * t8690 - t31055 - t31057 - t31060 + t31671 - t31733 + t31737 - t31746 - t31749 - t31750 - t8329;
    t32378
}
