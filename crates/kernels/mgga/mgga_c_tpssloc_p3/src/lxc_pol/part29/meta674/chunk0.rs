//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2262/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2262<F: Float>(t24987: F, t6880: F, t22573: F, t7684: F, t22575: F, t22585: F, t7685: F, t22607: F, t7754: F, t6875: F, t8944: F, t26164: F) -> (F, F, F, F, F) {
    let t91642 = F::new(6.0) * t24987 * t6880;
    let t91655 = t7684 * t22573;
    let t91657 = F::new(6.0) * t91655 * t22575;
    let t91662 = F::new(3.0) * t7685 * t22585;
    let t91666 = t22607 * t7754;
    let t91669 = t6875 * t8944;
    let t91671 = F::new(4.0) * t91669 * t26164;
    (t91642, t91657, t91662, t91666, t91671)
}
