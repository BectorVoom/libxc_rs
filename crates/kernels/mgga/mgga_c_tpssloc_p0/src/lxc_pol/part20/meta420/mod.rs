//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1830;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta420<F: Float>(t1025: F, t10403: F, t1041: F, t10413: F, t10909: F, t10923: F, t10927: F, t14174: F, t14180: F, t14184: F, t14189: F, t14194: F, t14198: F, t14203: F, t14207: F, t14215: F, t14222: F, t14230: F, t2960: F, t3070: F, t3117: F, t4590: F, t4609: F, t973: F, t14228: F, t4337: F, t10408: F, t13510: F, t13512: F, t13514: F, t13517: F, t13519: F, t13522: F, t13524: F, t13526: F, t13657: F, t13661: F, t13665: F, t13720: F, t13722: F, t13726: F, t13729: F, t13731: F, t13734: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10636: F, t13563: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F) -> (F, F, F, F, F) {
        let t14233 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1829::<F>(t1025, t10403, t1041, t10413, t10909, t10923, t10927, t14174, t14180, t14184, t14189, t14194, t14198, t14203, t14207, t14215, t14222, t14230, t2960, t3070, t3117, t4590, t4609, t973);
        let (t14234, t14235, t14238) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1830::<F>(t14228, t4337, t10408, t13510, t13512, t13514, t13517, t13519, t13522, t13524, t13526, t13657, t13661, t13665, t13720, t13722, t13726, t13729, t13731, t13734);
        let t14255 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1831::<F>(t13566, t13602, t10556, t10558, t10560, t10562, t10636, t13563, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13613);
    (t14233, t14234, t14235, t14238, t14255)
}
