//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1723;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1724;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta374<F: Float>(t131: F, t9558: F, t205: F, t221: F, t2379: F, t4128: F, t1489: F, t9541: F, t4126: F, t782: F, t4130: F, t12971: F, t210: F, t214: F, t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F, t2576: F, t787: F, t9572: F, t9574: F, t9579: F, t9583: F, t13003: F, t252: F, t1492: F, t2710: F, t1519: F, t2591: F, t225: F, t4266: F, t10049: F, t1528: F, t259: F, t2597: F, t2713: F, t2720: F, t2743: F, t4147: F, t4268: F, t4273: F, t4301: F, t866: F, t9590: F, t9593: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13004, t13005, t13007, t13010, t13012, t13014, t13017) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1723::<F>(t131, t9558, t205, t221, t2379, t4128, t1489, t9541, t4126, t782, t4130, t12971, t210, t214);
        let (t13025, t13028) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1724::<F>(t2563, t4138, t4134, t9546, t118, t4119, t794, t2576, t13005, t13007, t13010, t13014, t13017, t787, t9572, t9574, t9579, t9583);
        let (t13029, t13030, t13034, t13036, t13042, t13048) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1725::<F>(t13003, t13028, t252, t1492, t2710, t1519, t2591, t225, t4266, t10049, t1528, t259, t2597, t2713, t2720, t2743, t4147, t4268, t4273, t4301, t866, t9590, t9593);
    (t13004, t13005, t13007, t13012, t13017, t13025, t13029, t13030, t13034, t13036, t13042, t13048)
}
