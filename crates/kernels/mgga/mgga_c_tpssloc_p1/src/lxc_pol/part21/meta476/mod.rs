//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2065;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta476<F: Float>(t16131: F, t16435: F, t1378: F, t225: F, t5319: F, t1372: F, t5210: F, t12030: F, t12444: F, t1375: F, t1386: F, t16022: F, t16028: F, t16030: F, t1843: F, t3758: F, t3889: F, t3912: F, t5215: F, t5321: F, t5354: F, t568: F, t1385: F, t5353: F, t3887: F, t16413: F, t539: F, t5217: F, t1834: F, t3752: F, t1323: F, t5318: F, t16122: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16436, t16437, t16439, t16448, t16451) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2065::<F>(t16131, t16435, t1378, t225, t5319, t1372, t5210, t12030, t12444, t1375, t1386, t16022, t16028, t16030, t1843, t3758, t3889, t3912, t5215, t5321, t5354, t568);
        let (t16452, t16453, t16458, t16460, t16463, t16465, t16468) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2066::<F>(t1385, t5353, t3887, t16413, t539, t225, t5217, t1834, t3752, t1323, t5318, t16122, t562);
    (t16436, t16437, t16439, t16448, t16451, t16452, t16453, t16458, t16460, t16463, t16465, t16468)
}
