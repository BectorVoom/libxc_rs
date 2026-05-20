//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1874;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta398<F: Float>(t2844: F, t4395: F, t912: F, t2842: F, t2836: F, t4399: F, t10704: F, t1556: F, t2793: F, t10702: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10832: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14389, t14391, t14392, t14394, t14396, t14398, t14409, t14410) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1874::<F>(t2844, t4395, t912, t2842, t2836, t4399, t10704, t1556, t2793, t10702, t13566, t13602);
        let t14419 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1875::<F>(t10556, t10558, t10560, t10562, t10832, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613, t14409, t14410);
    (t14389, t14391, t14392, t14394, t14396, t14398, t14409, t14410, t14419)
}
