//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1356;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1357;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1358;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta225<F: Float>(t52: F, t2440: F, t5392: F, t5398: F, t76: F, t5512: F, t145: F, t185: F, t157: F, t182: F, t4200: F, t2373: F, t2377: F, t2408: F, t2417: F, t2522: F, t5497: F, t5498: F, t5501: F, t5502: F, t5506: F, zeta_threshold: F, t1484: F, t40: F, t75: F, t767: F, t771: F, t78: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t5519, t5520, t5521, t5522, t5524, t5525, t5526) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1356::<F>(t52, t2440, t5392, t5398, t76, t5512, t145, t185, t157, t182, t4200, t2373, t2377, t2408, t2417, t2522, t5497, t5498, t5501, t5502, t5506, zeta_threshold);
        let t5527 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1357::<F>(t1484);
        let t5544 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1358::<F>(t40, t52, t5392, t5398, t75, t767, t771, t78, zeta_threshold);
    (t5519, t5520, t5521, t5522, t5524, t5525, t5526, t5527, t5544)
}
