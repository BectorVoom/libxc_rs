//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2359/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2359<F: Float>(t22607: F, t7754: F, t6875: F, t8944: F, t26164: F, t1983: F, t22578: F, t7753: F, t7756: F, t531: F, t7752: F, t22596: F) -> (F, F, F, F, F) {
    let t91666 = t22607 * t7754;
    let t91669 = t6875 * t8944;
    let t91671 = F::cast_from(4.0_f64) * t91669 * t26164;
    let t91673 = t1983 * t7753 * t22578;
    let t91674 = t22607 * t7756;
    let t91675 = t531 * t7752;
    let t91678 = F::cast_from(6.0_f64) * t1983 * t91675 * t22596;
    (t91666, t91671, t91673, t91674, t91678)
}
