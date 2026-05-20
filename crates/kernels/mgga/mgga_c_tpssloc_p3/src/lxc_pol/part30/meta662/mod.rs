//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2084;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta662<F: Float>(t26392: F, t80670: F, t22705: F, t26422: F, t81228: F, t22704: F, t26466: F, t26461: F, t26433: F, t6883: F, t22716: F, t7741: F, t5336: F, t80798: F, t22724: F, t26436: F, t26423: F, t81159: F, t215: F, t22839: F, t562: F, t80854: F, t1338: F, t26328: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90837, t90845, t90860, t90865, t90867, t90868) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2084::<F>(t26392, t80670, t22705, t26422, t81228, t22704, t26466, t26461, t26433, t6883, t22716, t7741);
        let (t90899, t90900, t90913, t90914, t90915, t90952) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2085::<F>(t22704, t5336, t80798, t22724, t26436, t26423, t81159, t215, t22839, t562, t80854, t1338, t26328);
    (t90837, t90845, t90860, t90865, t90867, t90868, t90899, t90900, t90913, t90914, t90915, t90952)
}
