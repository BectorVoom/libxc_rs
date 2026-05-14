//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 735/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk735<F: Float>(t28: F, t265: F, t504: F, t24629: F, t24900: F, t3640: F, t7394: F, t11947: F, t2157: F, t1254: F, t1256: F, t193: F, t23772: F, t336: F, t3633: F, t3637: F, t4700: F, t7398: F, t2161: F, t2250: F, t23820: F, t52: F, t607: F, t7402: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t24901 = t24629 + t24900;
    let t24905 = t7394 * t3640;
    let t24909 = t2157 * t11947;
    let t24916 = piecewise3(t505, t1256 * t193 * t24901 * t336 - 2.0 * t1254 * t24905 * t4700 + 2.0 * t24909 * t3637 * t4700 - t3633 * t4700 * t7398, t23772);
    let t24923 = piecewise3(t401, t23820, t24916 * t52 / 2.0 - t7402 * t607 - t2161 * t2250 / 2.0);
    (t24923,)
}
