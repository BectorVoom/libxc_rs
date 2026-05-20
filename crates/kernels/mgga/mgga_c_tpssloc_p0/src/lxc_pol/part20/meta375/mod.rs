//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1726;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1727;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta375<F: Float>(t1527: F, t2719: F, t10110: F, t225: F, t4143: F, t2742: F, t2718: F, t4265: F, t798: F, t4145: F, t4142: F, t852: F, t4300: F, t865: F, t2684: F, t4180: F, t4181: F, t9646: F, t9647: F, t2633: F, t2645: F, t4248: F, t1496: F, t9541: F, t12850: F, t12860: F, t12861: F, t12889: F, t12891: F, t12894: F, t12906: F, t12910: F, t9457: F, t9462: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13050, t13053, t13059, t13062, t13065, t13068) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1726::<F>(t1527, t2719, t10110, t225, t4143, t2742, t2718, t4265, t798, t4145, t4142, t852);
        let (t13072, t13076, t13080, t13084, t13087) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1727::<F>(t4300, t865, t2718, t2684, t4180, t4181, t9646, t9647, t2633, t2645, t4248, t1496, t9541);
        let t13093 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1728::<F>(t12850, t12860, t12861, t12889, t12891, t12894, t12906, t12910, t9457, t9462, t9469, t9476, t9484, t9496, t9715);
    (t13050, t13053, t13059, t13062, t13065, t13068, t13072, t13076, t13080, t13084, t13087, t13093)
}
