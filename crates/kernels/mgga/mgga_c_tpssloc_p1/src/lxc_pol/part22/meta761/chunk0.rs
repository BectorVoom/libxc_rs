//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2562/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2562<F: Float>(t3264: F, t4782: F, t6020: F, t1671: F, t18834: F, t11185: F, t21899: F, t1670: F, t3313: F, t63588: F, t18258: F, t4781: F) -> (F, F, F, F, F) {
    let t71806 = F::new(6.0) * t3264 * t4782 * t6020;
    let t71809 = F::new(6.0) * t3264 * t1671 * t18834;
    let t71811 = F::cast_from(0.48245938496077605201e2_f64) * t11185 * t21899;
    let t71814 = F::cast_from(0.48245938496077605201e2_f64) * t3313 * t63588 * t1670;
    let t71817 = F::cast_from(0.48245938496077605201e2_f64) * t3313 * t18258 * t4781;
    (t71806, t71809, t71811, t71814, t71817)
}
