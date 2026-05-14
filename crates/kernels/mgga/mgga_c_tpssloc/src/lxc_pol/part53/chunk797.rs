//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 797/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk797<F: Float>(t28: F, t265: F, t504: F, t7109: F, t32071: F, t1081: F, t1877: F, t2522: F, t32030: F, t32034: F, t32047: F, t52: F, t607: F, t6841: F, t6848: F, t7114: F, t8744: F, t8748: F, t8770: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t32093 = t28 * t7109;
    let t32102 = piecewise3(t505, 0.0, t32071);
    let t32107 = piecewise3(t401, 3.0 / 2.0 * t2522 * t8744 * t6841 + t1877 * t32030 * t28 / 2.0 - t1877 * t32034 * t6848 / 2.0 + t1877 * t8744 * t1081 / 2.0 - 3.0 / 2.0 * t2522 * t8748 * t6841 - t1877 * t7114 * t32093 + t1877 * t32047 * t6848 - t1877 * t8748 * t1081 / 2.0, t32102 * t52 / 2.0 - t8770 * t607 / 2.0);
    (t32102, t32107)
}
