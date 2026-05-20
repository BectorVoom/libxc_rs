//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta441 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1687;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1688;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1689;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1690;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta441<F: Float>(t2240: F, t608: F, t1864: F, t645: F, t1863: F, t6489: F, t9231: F, t192: F, t532: F, t1982: F, t6995: F, t2018: F, t531: F, t1887: F, t6916: F, t213: F, t225: F, t562: F, t154: F, t835: F, t3748: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22549, t22550, t22551, t22554, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1687::<F>(t2240, t608, t1864, t645, t1863, t6489, t9231, t192, t532, t1982);
        let (t22591, t22595, t22633) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1688::<F>(t532, t6995, t2018, t531, t1887, t6916);
        let t22635 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1689::<F>(t213, t225, t562);
        let t22641 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1690::<F>(t154, t835);
        let t22642 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1691::<F>(t22641, t3748);
    (t22549, t22550, t22551, t22554, t22573, t22574, t22591, t22595, t22633, t22635, t22641, t22642)
}
