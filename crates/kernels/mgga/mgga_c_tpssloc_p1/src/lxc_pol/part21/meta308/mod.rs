//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1656;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1657;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta308<F: Float>(t11570: F, t2244: F, t3448: F, t3469: F, t2250: F, t3450: F, t3247: F, t460: F, t1176: F, t134: F, t1184: F) -> (F, F, F, F, F, F, F) {
        let (t11571, t11575, t11579, t11583) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1656::<F>(t11570, t2244, t3448, t3469, t2250, t3450, t3247, t460);
        let (t11584, t11588) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1657::<F>(t11583, t2244, t1176, t134);
        let t11589 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1658::<F>(t11588, t1184);
    (t11571, t11575, t11579, t11583, t11584, t11588, t11589)
}
