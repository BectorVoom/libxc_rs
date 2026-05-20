//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2040/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2040<F: Float>(t87197: F, t23145: F, t4166: F, t22690: F, t234: F, t7496: F, t776: F, t81792: F, t23109: F, t23110: F, t232: F, t236: F, t4233: F) -> (F, F, F, F, F) {
    let t87198 = F::new(7.0) / F::new(288.0) * t87197;
    let t87199 = t4166 * t23145;
    let t87202 = t22690 * t234;
    let t87205 = t81792 * t87202 * t7496 * t776;
    let t87206 = F::cast_from(0.28260929265898273598e-2_f64) * t87205;
    let t87211 = t23109 * t23110 * t236 * t4233 * t232;
    (t87198, t87199, t87202, t87206, t87211)
}
