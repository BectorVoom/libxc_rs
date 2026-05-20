//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk215;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk216;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk217;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk218;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk219;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk220;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta28<F: Float>(t219: F, t541: F, t555: F, t559: F, t539: F, t553: F, t544: F, t254: F, t144: F, t193: F, t523: F, t525: F, t533: F, t113: F, t510: F, t513: F, t111: F, t112: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t562 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk215::<F>(t219, t541, t555, t559);
        let (t563, t564) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk216::<F>(t539, t562, t553);
        let (t566, t568) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk217::<F>(t544, t564, t254);
        let (t570, t571) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk218::<F>(t563, t568);
        let t574 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk219::<F>(t144, t193, t523, t525, t533, t571);
        let t576 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk220::<F>(t113, t510, t513, t574);
        let t577 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk221::<F>(t111, t112);
    (t562, t563, t564, t566, t568, t570, t571, t574, t576, t577)
}
