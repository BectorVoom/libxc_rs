//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2169;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2170;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2171;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2172;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2173;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta650<F: Float>(t1649: F, t2553: F, t12971: F, t28: F, t1081: F, t4119: F, t13191: F, t25891: F, t25927: F, t57921: F, t13471: F, t1484: F, t3231: F, t86781: F, t1877: F, t1915: F, t22959: F, t23286: F, t23290: F, t25013: F, t2522: F, t25928: F, t25945: F, t6670: F, t7649: F, t86703: F, t86734: F, t86751: F, t86757: F, t87945: F, t23788: F, t86797: F, t16596: F, t83555: F, t4303: F, t40772: F, t86717: F, t23781: F, t23807: F, t23810: F, t23813: F, t25354: F, t25358: F, t25372: F, t25892: F, t25898: F, t25905: F, t4314: F, t6666: F, t6841: F, t7541: F, t81483: F, t86740: F, t86775: F, t86835: F, t87975: F, t25365: F, t1530: F, t2749: F, t57893: F, t2752: F, t13487: F, t23295: F, t23796: F, t25901: F, t25921: F, t25930: F, t25934: F, t25938: F, t47645: F, t7650: F, t7656: F, t81525: F, t89880: F, t23858: F, t7685: F, t22607: F, t7688: F, t1390: F, t16018: F, t1983: F, t6878: F, t22574: F, t56194: F, t8643: F, t12461: F, t6995: F, t26161: F, t26163: F, t22581: F, t24987: F, t7000: F, t25985: F, t6876: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t89881, t89888, t89892, t89896, t89904, t89907, t89911) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2169::<F>(t1649, t2553, t12971, t28, t1081, t4119, t13191, t25891, t25927, t57921, t13471, t1484, t3231);
        let t89920 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2170::<F>(t25927, t86781, t1877, t1915, t22959, t23286, t23290, t25013, t2522, t25928, t25945, t28, t6670, t7649, t86703, t86734, t86751, t86757, t87945, t89881, t89888, t89892, t89896, t89904, t89907, t89911);
        let t89957 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2171::<F>(t23788, t86797, t16596, t83555, t1081, t4303, t28, t40772, t86717, t1877, t22959, t23781, t23807, t23810, t23813, t25013, t2522, t25354, t25358, t25372, t25892, t25898, t25905, t4314, t6666, t6670, t6841, t7541, t81483, t86740, t86775, t86835, t87975);
        let t90001 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2172::<F>(t25365, t83555, t1530, t3231, t1649, t2749, t23788, t57893, t2752, t13487, t1877, t22959, t23286, t23290, t23295, t23796, t2522, t25901, t25921, t25930, t25934, t25938, t47645, t6666, t6670, t7541, t7650, t7656, t81483, t81525);
        let (t90003, t90020, t90022, t90026) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2173::<F>(t89880, t89920, t89957, t90001, t23858, t7685, t22607, t7688, t1390, t16018, t1983, t6878);
        let (t90029, t90034, t90036, t90038, t90040) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2174::<F>(t22574, t56194, t8643, t12461, t6995, t26161, t26163, t22581, t7685, t24987, t7000, t25985, t6876);
    (t90003, t90020, t90022, t90026, t90029, t90034, t90036, t90038, t90040)
}
