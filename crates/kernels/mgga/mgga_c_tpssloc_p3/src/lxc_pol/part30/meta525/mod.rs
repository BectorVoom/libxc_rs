//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta525<F: Float>(t26193: F, t6907: F, t1985: F, t225: F, t5318: F, t567: F, t214: F, t1377: F, t1842: F, t1307: F, t22635: F, t22633: F, t2006: F, t5210: F, t1807: F, t6955: F, t22646: F, t26184: F, t26187: F, t26191: F, t26195: F, t26198: F, t26200: F, t26204: F, t568: F) -> (F, F, F, F, F, F, F, F) {
        let (t26206, t26207, t26210, t26211, t26212, t26215, t26216, t26217) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1865::<F>(t26193, t6907, t1985, t225, t5318, t567, t214, t1377, t1842, t1307, t22635, t22633);
        let (t26219, t26221, t26223) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1866::<F>(t2006, t5210, t1807, t6955, t22646, t26184, t26187, t26191, t26195, t26198, t26200, t26204, t26207, t26212, t26217, t568);
    (t26206, t26210, t26211, t26215, t26216, t26219, t26221, t26223)
}
