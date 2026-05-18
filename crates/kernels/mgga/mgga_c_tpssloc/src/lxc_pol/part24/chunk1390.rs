//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1390/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1390<F: Float>(t10472: F, t10474: F, t10478: F, t23535: F, t10948: F, t23540: F, t6753: F, t10961: F, t6754: F, t3077: F, t6764: F, t1937: F, t607: F, t6722: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    let t83061 = t10948 * t23540;
    let t83065 = t10472 * t6753 * t10478;
    let t83068 = t10961 * t6754;
    let t83071 = t3077 * t6764;
    let t83075 = t6722 * t607 * t1937;
    (t83054, t83058, t83061, t83065, t83068, t83071, t83075)
}
