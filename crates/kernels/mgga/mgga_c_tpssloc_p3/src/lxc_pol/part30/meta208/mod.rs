//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk981;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta208<F: Float>(t40: F, t4100: F, t4102: F, t185: F, t5392: F, t2658: F, t1484: F, t4310: F, t1462: F, t4205: F, t2433: F, t5398: F, t73: F, zeta_threshold: F, t52: F, t2440: F, t76: F, t145: F, t157: F, t182: F, t4200: F, t2373: F, t2377: F, t2408: F, t2417: F, t2522: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5497, t5498, t5499, t5501, t5502, t5506, t5512) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk981::<F>(t40, t4100, t4102, t185, t5392, t2658, t1484, t4310, t1462, t4205, t2433, t5398, t73, zeta_threshold);
        let (t5519, t5520, t5521, t5522, t5524, t5525, t5526) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk982::<F>(t52, t2440, t5392, t5398, t76, t5512, t145, t185, t157, t182, t4200, t2373, t2377, t2408, t2417, t2522, t5497, t5498, t5501, t5502, t5506, zeta_threshold);
    (t5497, t5498, t5499, t5501, t5506, t5519, t5520, t5521, t5522, t5524, t5525, t5526)
}
