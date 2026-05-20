//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta483<F: Float>(t26193: F, t6907: F, t1985: F, t225: F, t5318: F, t567: F, t214: F, t1377: F, t1842: F, t1307: F, t22635: F, t22633: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26206, t26207, t26210, t26211, t26212, t26214, t26215, t26216, t26217) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1697::<F>(t26193, t6907, t1985, t225, t5318, t567, t214, t1377, t1842, t1307, t22635, t22633);
    (t26206, t26207, t26210, t26211, t26212, t26214, t26215, t26216, t26217)
}
