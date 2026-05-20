//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk983;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk984;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk985;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta209<F: Float>(t1484: F, t40: F, t52: F, t5392: F, t5398: F, t75: F, t767: F, t771: F, t78: F, zeta_threshold: F, t210: F, t214: F, t2562: F, t2569: F, t2571: F, t2590: F, t4124: F, t4135: F, t787: F, t252: F, t1492: F, t1519: F, t119: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5527 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk983::<F>(t1484);
        let t5544 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk984::<F>(t40, t52, t5392, t5398, t75, t767, t771, t78, zeta_threshold);
        let (t5550, t5555, t5558) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk985::<F>(t210, t214, t5527, t5544, t2562, t2569, t2571, t2590, t4124, t4135, t787);
        let (t5559, t5561, t5567, t5568, t5571, t5572, t5575) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk986::<F>(t252, t5558, t1492, t1519, t119, t5527, t210, t5544, t225);
    (t5527, t5544, t5550, t5555, t5558, t5559, t5561, t5567, t5568, t5571, t5572, t5575)
}
