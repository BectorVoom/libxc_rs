//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1032;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta302<F: Float>(t10996: F, t20234: F, t974: F, t1616: F, t5685: F, t3071: F, t5677: F, t10408: F, t1539: F, t5867: F, t21118: F, t248: F, t3062: F, t21238: F, t942: F, t951: F, t959: F, t21093: F, t21097: F, t21099: F, t21103: F, t21105: F, t21107: F, t21365: F, t21367: F, t21369: F, t21375: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21561, t21562, t21565, t21566, t21569, t21570, t21573, t21574, t21580) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1032::<F>(t10996, t20234, t974, t1616, t5685, t3071, t5677, t10408, t1539, t5867, t21118, t248, t3062);
        let (t21589, t21591, t21592) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1033::<F>(t21238, t942, t951, t959, t21093, t21097, t21099, t21103, t21105, t21107, t21365, t21367, t21369, t21375);
    (t21561, t21562, t21565, t21566, t21569, t21570, t21573, t21574, t21580, t21589, t21591, t21592)
}
