//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1993/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1993<F: Float>(t100638: F, t100641: F, t100644: F, t100656: F, t100669: F, t100696: F, t100731: F, t100769: F, t100791: F, t101833: F, t101840: F, t1081: F, t1877: F, t24191: F, t24344: F, t25928: F, t25930: F, t26563: F, t26744: F, t26756: F, t28: F, t28771: F, t29106: F, t7114: F, t84797: F) -> F {
    let t101981 = F::new(2.0) * t101840 * t25928 - F::new(3.0) * t24191 * t100769 - F::new(3.0) / F::new(2.0) * t24191 * t100731 + t1877 * t101833 * t28 / F::new(2.0) - F::new(3.0) * t26563 * t100638 - F::new(3.0) * t24191 * t100656 + t26756 * t100644 + t1877 * t24344 * t100669 - F::new(3.0) * t84797 * t28771 + t1877 * t29106 * t1081 / F::new(2.0) - t1877 * t7114 * t100696 + F::new(3.0) * t26563 * t100641 + F::new(3.0) * t24191 * t100791 - t1877 * t26744 * t25930;
    t101981
}
