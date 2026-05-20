//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2081;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta660<F: Float>(t6897: F, t6907: F, t90544: F, t26203: F, t6883: F, t7700: F, t80645: F, t225: F, t26219: F, t214: F, t5318: F, t26378: F, t6914: F, t1372: F, t1799: F, t26411: F, t22704: F, t22705: F, t5345: F, t22690: F, t552: F, t26447: F, t90607: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90702, t90708, t90724, t90732, t90739, t90749) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2081::<F>(t6897, t6907, t90544, t26203, t6883, t7700, t80645, t225, t26219, t214, t5318, t26378, t6914);
        let (t90750, t90754, t90760, t90782, t90787, t90789) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2082::<F>(t90749, t1372, t1799, t26411, t6914, t22704, t22705, t5345, t22690, t552, t26447, t90607);
    (t90702, t90708, t90724, t90732, t90739, t90750, t90754, t90760, t90782, t90787, t90789)
}
