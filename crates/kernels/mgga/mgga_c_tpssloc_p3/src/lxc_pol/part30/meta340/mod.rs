//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1373;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta340<F: Float>(t1509: F, t828: F, t2632: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F, t2642: F, t4166: F, t2628: F, t836: F, t812: F, t4184: F, t242: F, t9972: F, t2639: F, t4236: F, t1512: F, t9674: F, t2638: F, t831: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13223, t13228, t13234, t13237, t13242, t13251, t13257) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1373::<F>(t1509, t828, t2632, t1500, t2693, t4163, t838, t120, t4233, t2642, t4166, t2628, t836);
        let (t13260, t13262, t13275, t13277, t13278, t13280) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1374::<F>(t13257, t812, t4184, t242, t9972, t2639, t4236, t1512, t9674, t2638, t4166, t831);
    (t13223, t13228, t13234, t13237, t13242, t13251, t13260, t13262, t13275, t13277, t13278, t13280)
}
