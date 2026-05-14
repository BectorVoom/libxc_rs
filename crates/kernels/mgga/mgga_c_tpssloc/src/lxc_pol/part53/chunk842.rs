//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 842/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk842<F: Float>(t23110: F, t23185: F, t31385: F, t22690: F, t23171: F, t31376: F, t31389: F, t6562: F, t794: F, t23012: F, t8557: F, t234: F, t7084: F, t112778: F, t112803: F, t112818: F) -> (F, F, F, F, F, F, F, F) {
    let t114680 = t23185 * t23110 * t31385;
    let t114688 = t23171 * t22690 * t31376;
    let t114691 = t6562 * t794 * t31389;
    let t114693 = t23012 * t8557;
    let t114696 = t234 * t7084;
    let t114714 = 0.5383034145885385447e-3 * t112778;
    let t114720 = 7.0 / 576.0 * t112803;
    let t114724 = 0.32298204875312312682e-2 * t112818;
    (t114680, t114688, t114691, t114693, t114696, t114714, t114720, t114724)
}
