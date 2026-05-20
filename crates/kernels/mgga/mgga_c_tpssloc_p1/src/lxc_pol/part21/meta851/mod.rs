//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta851 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3079;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta851<F: Float>(t3271: F, t43889: F, t5992: F, t11243: F, t5999: F, t43880: F, t11265: F, t63323: F, t63327: F, t63330: F, t63848: F, t63853: F, t63856: F, t63858: F, t63860: F, t63862: F, t63865: F, t63867: F, t18520: F, t699: F, t2403: F, t6011: F, t136: F, t3297: F, t63357: F, t6014: F, t1113: F, t63363: F, t44938: F, t48140: F, t55716: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t63870, t63873, t63876, t63879, t63881) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3079::<F>(t3271, t43889, t5992, t11243, t5999, t43880, t11265, t63323, t63327, t63330, t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867);
        let (t63886, t63888, t63891, t63893, t63896, t63899) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3080::<F>(t18520, t699, t2403, t6011, t136, t3297, t63357, t6014, t1113, t63363, t44938, t48140, t55716);
    (t63870, t63873, t63876, t63879, t63881, t63886, t63888, t63891, t63893, t63896, t63899)
}
