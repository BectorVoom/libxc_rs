//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1869;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1870;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta395<F: Float>(t13644: F, t13602: F, t13598: F, t13613: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13647: F, t10300: F, t10556: F, t10558: F, t10560: F, t10562: F, t10784: F, t10785: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13552: F, t13557: F, t13561: F, t13616: F, t13624: F, t13626: F, t14287: F, t14291: F, t14304: F, t932: F, t4446: F, t942: F, t1573: F, t2929: F, t13716: F, t951: F, t13563: F, t13566: F, t10608: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14321, t14324, t14328) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1869::<F>(t13644, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642, t13647, t10300, t10556, t10558, t10560, t10562, t10784, t10785, t13530, t13534, t13539, t13544, t13548, t13552, t13557, t13561, t13616, t13624, t13626, t14287, t14291, t14304);
        let (t14329, t14332, t14337) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1870::<F>(t14328, t932, t4446, t942, t1573, t2929);
        let (t14344, t14352, t14353, t14354, t14363) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1871::<F>(t13716, t951, t13563, t13566, t13602, t10556, t10558, t10560, t10562, t10608, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
    (t14321, t14324, t14328, t14329, t14332, t14337, t14344, t14352, t14353, t14354, t14363)
}
