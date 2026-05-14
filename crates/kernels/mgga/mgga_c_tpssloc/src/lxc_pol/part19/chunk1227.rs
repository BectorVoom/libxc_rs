//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1227/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1227<F: Float>(t10273: F, t2960: F, t10231: F, t10279: F, t973: F, t10186: F, t10235: F, t10237: F, t10238: F, t10242: F, t13798: F, t2986: F, t2991: F, t41693: F, t42827: F, t42830: F, t42833: F, t42839: F, t42842: F, t42846: F) -> (F,) {
    let t42855 = t2960 * t10273;
    let t42858 = t973 * t10231 * t10279;
    let t42860 = 0.14814814814814814815e-2 * t42827 - 0.32592592592592592592e-1 * t42830 * t2991 + 0.59259259259259259256e-2 * t42833 + 0.11851851851851851852e-1 * t10186 * t10238 - 0.14814814814814814814e-2 * t42839 + 0.88888888888888888886e-2 * t2986 * t10235 * t42842 - 0.22222222222222222222e-2 * t2986 * t42846 * t10237 + 0.88888888888888888887e-2 * t10186 * t10242 + 0.51851851851851851851e-2 * t2986 * t13798 * t41693 + 0.59259259259259259256e-2 * t42855 - 0.29629629629629629628e-2 * t42858;
    (t42860,)
}
