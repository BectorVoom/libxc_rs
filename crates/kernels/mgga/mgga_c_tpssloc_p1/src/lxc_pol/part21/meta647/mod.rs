//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2441;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta647<F: Float>(t340: F, t625: F, t221: F, t339: F, t344: F, t1887: F, t2262: F, t337: F, t13783: F, t984: F, t10277: F, t343: F, t3014: F, t4509: F, t42308: F, t974: F, t10224: F, t2999: F, t973: F, t2978: F, t698: F, t2981: F, t2402: F, t976: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42813, t42817, t42830, t42837, t42841) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2441::<F>(t340, t625, t221, t339, t344, t1887, t2262, t337, t13783, t984, t10277, t343);
        let (t42846, t42861, t42873, t42875, t42877, t42891) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2442::<F>(t3014, t4509, t42308, t974, t10224, t2999, t973, t2978, t698, t2981, t2402, t976);
    (t42813, t42817, t42830, t42837, t42841, t42846, t42861, t42873, t42875, t42877, t42891)
}
