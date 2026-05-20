//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta451<F: Float>(t19755: F, t20021: F, t1378: F, t1385: F, t6460: F, t3887: F, t225: F, t6364: F, t20009: F, t539: F, t1375: F, t1386: F, t16030: F, t16439: F, t1843: F, t19635: F, t19644: F, t19648: F, t3882: F, t5321: F, t5326: F, t5354: F, t568: F, t6461: F) -> (F, F, F, F, F, F, F) {
        let (t20022, t20023, t20025, t20026, t20029, t20032, t20034) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1811::<F>(t19755, t20021, t1378, t1385, t6460, t3887, t225, t6364, t20009, t539, t1375, t1386, t16030, t16439, t1843, t19635, t19644, t19648, t3882, t5321, t5326, t5354, t568, t6461);
    (t20022, t20023, t20025, t20026, t20029, t20032, t20034)
}
