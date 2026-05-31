//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 744/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk744<F: Float>(t22792: F, t22794: F, t547: F, t6546: F, t1329: F, t2230: F, t6924: F, t213: F, t6928: F, t10: F, t2229: F, t60: F) -> (F, F, F, F, F, F) {
    let t22795 = t22792 * t22794;
    let t22797 = t6546 * t547;
    let t22798 = t22797 * t1329;
    let t22799 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t22798;
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    let t22805 = t22804 * t6928;
    let t22811 = t2229 * t10;
    let t22813 = F::cast_from(1.0_f64) / t60 / t22811;
    (t22795, t22797, t22799, t22804, t22805, t22813)
}
