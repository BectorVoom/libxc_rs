//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1283/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1283<F: Float>(t22893: F, t23164: F, t31377: F, t31390: F, t6547: F, t23030: F, t31381: F, t23110: F, t23185: F, t31385: F, t22690: F, t23171: F, t31376: F) -> (F, F, F, F, F) {
    let t114666 = t23164 * t22893 * t31377;
    let t114670 = t6547 * t31390;
    let t114672 = t23030 * t31381;
    let t114673 = F::cast_from(0.26044789391763585244e-1_f64) * t114672;
    let t114680 = t23185 * t23110 * t31385;
    let t114688 = t23171 * t22690 * t31376;
    (t114666, t114670, t114673, t114680, t114688)
}
