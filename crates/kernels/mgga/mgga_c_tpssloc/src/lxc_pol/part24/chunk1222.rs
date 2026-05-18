//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1222/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1222<F: Float>(t22588: F, t23861: F, t3: F, t112: F, t7002: F, t111: F, t2022: F, t12521: F, t1873: F, t12524: F, t7015: F, t3938: F, t6534: F) -> (F, F, F, F, F, F, F) {
    let t23862 = t22588 + t23861;
    let t23863 = t3 * t23862;
    let t23877 = t7002 * t112;
    let t23880 = t2022 * t111;
    let t23886 = F::new(0.135e2) * t12521 * t1873;
    let t23888 = F::new(54.0) * t12524 * t7015;
    let t23890 = F::new(27.0) * t3938 * t6534;
    (t23862, t23863, t23877, t23880, t23886, t23888, t23890)
}
