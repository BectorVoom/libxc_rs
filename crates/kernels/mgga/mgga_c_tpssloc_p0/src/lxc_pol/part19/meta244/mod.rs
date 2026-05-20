//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk977;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk978;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk979;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta244<F: Float>(t11282: F, t440: F, t11129: F, t11285: F, t11135: F, t11203: F, t11161: F, t11170: F, t11197: F, t11200: F, t11206: F, t11209: F, t11211: F, t11213: F, t11215: F, t11217: F, t11221: F, t11224: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F, t11259: F, t11261: F, t11266: F, t1137: F, t1127: F, t3355: F, t427: F, t3358: F, t435: F, t11306: F, t1147: F, t3368: F, t1143: F, t3400: F, t11182: F, t11184: F, t11187: F, t11194: F, t11272: F, t11280: F, t1129: F, t11297: F, t11300: F, t11303: F, t11307: F, t1157: F, t3334: F, t3357: F, t3371: F, t3378: F, t3396: F, t3401: F, t3404: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11310, t11311, t11328) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk977::<F>(t11282, t440, t11129, t11285, t11135, t11203, t11161, t11170, t11197, t11200, t11206, t11209, t11211, t11213, t11215, t11217, t11221, t11224);
        let t11343 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk978::<F>(t11137, t11139, t11141, t11143, t11150, t11156, t11165, t11174, t11230, t11233, t11245, t11259, t11261, t11266);
        let (t11344, t11345, t11349, t11350, t11352, t11353, t11356, t11361) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk979::<F>(t11328, t11343, t1137, t1127, t3355, t427, t3358, t435, t11306, t1147, t3368, t1143, t3400);
        let t11364 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk980::<F>(t11182, t11184, t11187, t11194, t11272, t11280, t1129, t11297, t11300, t11303, t11307, t11310, t11311, t11345, t11350, t11353, t11356, t11361, t1157, t3334, t3357, t3371, t3378, t3396, t3401, t3404);
    (t11310, t11311, t11344, t11345, t11349, t11350, t11352, t11353, t11356, t11361, t11364)
}
