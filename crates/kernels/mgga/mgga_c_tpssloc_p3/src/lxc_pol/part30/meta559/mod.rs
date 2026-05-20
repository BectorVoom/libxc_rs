//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta559<F: Float>(t23270: F, t28267: F, t22986: F, t225: F, t258: F, t5631: F, t214: F, t1880: F, t5544: F, t6554: F, t6553: F, t6552: F) -> (F, F, F, F, F, F, F, F) {
        let (t28268, t28269, t28272, t28273, t28274, t28276, t28277, t28278) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1919::<F>(t23270, t28267, t22986, t225, t258, t5631, t214, t1880, t5544, t6554, t6553, t6552);
    (t28268, t28269, t28272, t28273, t28274, t28276, t28277, t28278)
}
