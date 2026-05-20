//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk895;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk896;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta168<F: Float>(t25: F, t1268: F, t2312: F, t2314: F, t2319: F, t2363: F, t671: F, t88: F, t526: F, t606: F, t2249: F, t514: F, t528: F, zeta_threshold: F, t28: F, t1081: F, t3231: F, t517: F, t157: F, t182: F, t118: F, t521: F) -> (F, F, F, F, F, F, F, F) {
        let (t3660, t3664, t3665, t3671, t3672) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk895::<F>(t25, t1268, t2312, t2314, t2319, t2363, t671, t88, t526, t606, t2249, t514, t528, zeta_threshold);
        let (t3673, t3681) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk896::<F>(t28, t1081, t3231, t3672, t517, t157, t3671, zeta_threshold);
        let (t3683, t3684) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk897::<F>(t182, t3681, t118, t521);
    (t3660, t3664, t3665, t3672, t3673, t3681, t3683, t3684)
}
