//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2131;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2132;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2133;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2134;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2135;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta633<F: Float>(t25041: F, t87049: F, t215: F, t6581: F, t252: F, t81613: F, t13224: F, t23056: F, t13352: F, t25242: F, t6579: F, t25245: F, t82031: F, t81575: F, t25038: F, t4282: F, t6646: F, t9647: F, t25251: F, t23012: F, t7529: F, t13380: F, t22986: F, t2647: F, t13377: F, t1880: F, t1894: F, t214: F, t22984: F, t22992: F, t22993: F, t23009: F, t25297: F, t2617: F, t4166: F, t4234: F, t812: F, t81571: F, t81592: F, t1888: F, t232: F, t47448: F, t23110: F, t23185: F, t25241: F, t25248: F, t25249: F, t2553: F, t1519: F, t2631: F, t1484: F, t852: F, t776: F, t13393: F, t22996: F, t81595: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87050, t87052, t87055, t87059, t87067, t87068) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2131::<F>(t25041, t87049, t215, t6581, t252, t81613, t13224, t23056, t13352, t25242, t6579, t25245, t82031);
        let (t87073, t87076, t87078, t87080, t87084) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2132::<F>(t81575, t25038, t4282, t6646, t9647, t25251, t87049, t23012, t7529, t13380, t22986, t2647);
        let t87094 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2133::<F>(t13377, t1880, t1894, t214, t22984, t22992, t22993, t23009, t25297, t2617, t4166, t4234, t812, t81571, t81592, t87055, t87059, t87067, t87068, t87073, t87076, t87078, t87080, t87084);
        let (t87097, t87101, t87104, t87106) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2134::<F>(t1888, t232, t47448, t6646, t23110, t23185, t25241, t25038, t25248, t25249, t2553, t1519, t2631);
        let (t87109, t87111, t87114, t87117, t87119) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2135::<F>(t1888, t232, t6646, t87106, t1484, t852, t25038, t25248, t776, t13393, t22996, t81595);
    (t87050, t87052, t87094, t87097, t87101, t87104, t87106, t87109, t87111, t87114, t87117, t87119)
}
