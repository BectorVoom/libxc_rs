//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta556<F: Float>(t10143: F, t7109: F, t82069: F, t81598: F, t81735: F, t81742: F, t81849: F, t81852: F, t81920: F, t81954: F, t24234: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t84800, t84820, t84851, t84857, t84859, t84896, t84897, t84921, t84932, t84945) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1827::<F>(t10143, t7109, t82069, t81598, t81735, t81742, t81849, t81852, t81920, t81954, t24234, t814);
    (t84800, t84820, t84851, t84857, t84859, t84896, t84897, t84921, t84932, t84945)
}
