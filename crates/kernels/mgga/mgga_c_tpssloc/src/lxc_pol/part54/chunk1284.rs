//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1284/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1284<F: Float>(t31390: F, t6547: F, t23030: F, t31381: F, t23110: F, t23185: F, t31385: F, t22690: F, t23171: F, t31376: F, t31389: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t114670 = t6547 * t31390;
    let t114672 = t23030 * t31381;
    let t114673 = F::cast_from(0.26044789391763585244e-1_f64) * t114672;
    let t114680 = t23185 * t23110 * t31385;
    let t114688 = t23171 * t22690 * t31376;
    let t114689 = F::cast_from(0.82246703342411321824e-2_f64) * t114688;
    let t114691 = t6562 * t794 * t31389;
    (t114670, t114673, t114680, t114689, t114691)
}
