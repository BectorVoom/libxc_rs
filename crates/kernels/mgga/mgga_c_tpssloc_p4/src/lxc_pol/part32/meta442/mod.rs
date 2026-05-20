//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1691;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta442<F: Float>(t3792: F, t6414: F, t2632: F, t5611: F, t107: F, t240: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t2240: F, t608: F, t1864: F, t645: F, t192: F, t532: F, t1982: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20473, t20986, t22469, t22470, t22471, t22473, t22510, t22549) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1691::<F>(t3792, t6414, t2632, t5611, t107, t240, t625, t656, t666, t2331, t63, t2240, t608);
        let (t22550, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1692::<F>(t1864, t645, t192, t532, t1982);
    (t20473, t20986, t22469, t22470, t22471, t22473, t22510, t22549, t22550, t22573, t22574)
}
