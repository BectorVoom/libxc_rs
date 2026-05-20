//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1468;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1469;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta365<F: Float>(t13242: F, t4180: F, t4182: F, t4181: F, t9632: F, t2642: F, t4166: F, t2617: F, t4177: F, t2628: F, t836: F, t812: F, t4184: F, t242: F, t9972: F, t2631: F, t9975: F, t13225: F, t13231: F, t13234: F, t13237: F, t2643: F, t2649: F, t4178: F, t4191: F, t4240: F, t9639: F, t9642: F, t9668: F, t9672: F, t9675: F, t9679: F, t9986: F, t9988: F, t9994: F, t2639: F, t4236: F, t1512: F, t9674: F, t2638: F, t831: F, t2629: F, t4250: F, t9638: F, t1495: F, t210: F, t2379: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13244, t13248, t13251, t13254, t13258) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1468::<F>(t13242, t4180, t4182, t4181, t9632, t2642, t4166, t2617, t4177, t2628, t836, t812);
        let (t13263, t13265, t13268) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1469::<F>(t13258, t4184, t242, t9972, t812, t2631, t9975, t4180, t4181, t13225, t13231, t13234, t13237, t13244, t13248, t13251, t13254, t2643, t2649, t4178, t4191, t4240, t9639, t9642, t9668, t9672, t9675, t9679, t9986, t9988, t9994);
        let (t13275, t13277, t13280, t13283, t13287, t13289) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1470::<F>(t2639, t4236, t1512, t9674, t2638, t4166, t831, t2629, t4250, t9638, t1495, t210, t2379);
    (t13244, t13248, t13263, t13265, t13268, t13275, t13277, t13280, t13283, t13287, t13289)
}
