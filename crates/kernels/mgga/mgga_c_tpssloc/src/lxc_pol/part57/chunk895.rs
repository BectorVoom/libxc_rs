//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 895/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk895<F: Float>(t33620: F, t652: F, t2095: F, t33136: F, t1983: F, t1774: F, t1869: F, t2036: F, t2075: F, t33579: F, t33601: F, t33605: F, t33611: F, t33615: F, t33619: F, t510: F, t574: F, t7451: F, t7670: F, t7890: F, t7904: F, t7943: F, t8450: F, t8519: F) -> (F, F) {
    let t33622 = F::new(2.0) * t652 * t33620;
    let t33623 = t2095 * t33136;
    let t33624 = t1983 * t33623;
    let t33625 = -t1774 * t8519 - t1869 * t7890 - t2036 * t7670 - t2075 * t7451 - t33579 * t510 + t33601 * t574 + F::new(3.0) * t7904 * t8450 - t7943 * t8450 + t33605 - t33611 + t33615 - t33619 - t33622 - t33624;
    (t33623, t33625)
}
