//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 767/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk767<F: Float>(t29816: F, t7285: F, t27820: F, t8002: F, t1238: F, t24589: F, t27808: F, t27818: F, t29795: F, t29798: F, t29804: F, t29809: F, t29813: F, t5055: F, t6268: F, t7283: F, t7351: F, t8088: F) -> (F,) {
    let t29817 = t7285 * t29816;
    let t29822 = t27820 * t8002;
    let t29825 = -0.14621636149762012769e-1 * t27808 - t1238 * t29795 - 6.0 * t1238 * t29798 - t7351 * t6268 + 0.54831135561607547884e-2 * t27818 + 0.16449340668482264365e-1 * t7283 * t29804 + 0.54831135561607547884e-2 * t24589 * t29809 - 0.27415567780803773942e-2 * t7283 * t29813 - 0.54831135561607547884e-2 * t7283 * t29817 - 2.0 * t5055 * t8088 + 0.54831135561607547884e-2 * t24589 * t29822;
    (t29825,)
}
