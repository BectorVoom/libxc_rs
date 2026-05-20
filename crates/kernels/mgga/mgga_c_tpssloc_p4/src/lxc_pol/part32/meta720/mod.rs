//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta720 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2287;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2288;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta720<F: Float>(t55353: F, t7769: F, t16521: F, t7467: F, t1873: F, t19534: F, t3941: F, t28017: F, t671: F, t20173: F, t28899: F, t1395: F, t5456: F, t20162: F, t6534: F, t26545: F, t33185: F, t12524: F, t28896: F, t5493: F, t2174: F, t6470: F, t25: F, t265: F, t394: F, t100624: F, t1409: F, t16558: F, t2116: F, t27373: F, t29507: F, t3966: F, t40: F, t5398: F, t607: F, t7274: F, t7992: F, t99069: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t100917, t100921, t100924, t100927, t100929, t100930) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2287::<F>(t55353, t7769, t16521, t7467, t1873, t19534, t3941, t28017, t671, t20173, t28899, t1395, t5456);
        let (t100932, t100934, t100936, t100938, t100941, t103103) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2288::<F>(t100930, t1873, t20162, t6534, t26545, t33185, t12524, t28896, t3941, t5493, t2174, t6470);
        let t103125 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2289::<F>(t25, t265, t394, t100624, t1409, t16558, t2116, t27373, t29507, t3966, t40, t5398, t607, t7274, t7992, t99069, dens_threshold, rho0, zeta_threshold);
    (t100917, t100921, t100924, t100927, t100929, t100932, t100934, t100936, t100938, t100941, t103103, t103125)
}
