//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2057;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta645<F: Float>(t1611: F, t23528: F, t23436: F, t4640: F, t14507: F, t23536: F, t23540: F, t23433: F, t4630: F, t10189: F, t1920: F, t4343: F, t13783: F, t4338: F, t14192: F, t6717: F, t13965: F, t6755: F, t25577: F, t3103: F, t1933: F, t23479: F, t88405: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t88584, t88591, t88594, t88600, t88604, t88622) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2057::<F>(t1611, t23528, t23436, t4640, t14507, t23536, t23540, t23433, t4630, t10189, t1920, t4343);
        let (t88625, t88636, t88645, t88648, t88689) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2058::<F>(t13783, t1920, t4338, t14192, t6717, t13965, t6755, t25577, t3103, t1933, t23479, t88405);
    (t88584, t88591, t88594, t88600, t88604, t88622, t88625, t88636, t88645, t88648, t88689)
}
