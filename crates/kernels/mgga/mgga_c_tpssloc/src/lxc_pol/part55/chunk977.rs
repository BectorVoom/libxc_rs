//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 977/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk977<F: Float>(t28: F, t265: F, t504: F, t32507: F, t32550: F, t3640: F, t8900: F, t11947: F, t8904: F, t1254: F, t1256: F, t193: F, t30952: F, t336: F, t4700: F, t7394: F, t7398: F, t30982: F, t52: F, t607: F, t8909: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t32551 = t32507 + t32550;
    let t32555 = t8900 * t3640;
    let t32561 = t8904 * t11947;
    let t32566 = piecewise3(t505, t1256 * t193 * t32551 * t336 - t1254 * t32555 * t4700 + 2.0 * t1254 * t32561 * t4700 - 2.0 * t4700 * t7394 * t7398, t30952);
    let t32571 = piecewise3(t401, t30982, t32566 * t52 / 2.0 - t8909 * t607 / 2.0);
    (t32551, t32555, t32561, t32566, t32571)
}
