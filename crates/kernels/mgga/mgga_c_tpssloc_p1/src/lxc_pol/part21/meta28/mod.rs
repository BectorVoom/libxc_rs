//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk211;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk212;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk213;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk214;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk215;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk216;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk217;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta28<F: Float>(t550: F, t68: F, t236: F, t544: F, t532: F, t242: F, t248: F, t219: F, t541: F, t539: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t551, t552, t553) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk211::<F>(t550, t68);
        let t554 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk212::<F>(t236, t553);
        let t555 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk213::<F>(t544, t554);
        let t556 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk214::<F>(t532);
        let t557 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk215::<F>(t556);
        let t559 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk216::<F>(t242, t248, t557);
        let t562 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk217::<F>(t219, t541, t555, t559);
        let (t563, t564) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk218::<F>(t539, t562, t553);
    (t551, t552, t553, t554, t555, t556, t557, t559, t562, t563, t564)
}
