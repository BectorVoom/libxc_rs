//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1848;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta429<F: Float>(t13953: F, t14004: F, t14050: F, t14074: F, t14120: F, t14170: F, t14233: F, t14523: F, t349: F, t225: F, t4658: F, t1625: F, t3020: F, t1603: F, t3166: F, t13939: F, t381: F, t1049: F, t4552: F, t1052: F, t1066: F, t13736: F, t13743: F, t3026: F, t3169: F, t3207: F, t388: F, t4660: F, t4665: F, t4694: F) -> (F, F, F, F, F, F, F, F) {
        let (t14526, t14527, t14529, t14532) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1848::<F>(t13953, t14004, t14050, t14074, t14120, t14170, t14233, t14523, t349, t225, t4658, t1625, t3020);
        let (t14534, t14536, t14538, t14543) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1849::<F>(t1603, t3166, t13939, t381, t1049, t4552, t1052, t1066, t13736, t13743, t14527, t14529, t14532, t3026, t3169, t3207, t388, t4660, t4665, t4694);
    (t14526, t14527, t14529, t14532, t14534, t14536, t14538, t14543)
}
