//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1756/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1756<F: Float>(t225: F, t6151: F, t6153: F, t6239: F, t1720: F, t5052: F, t1751: F, t4940: F, t18571: F, t491: F, t1252: F, t14972: F, t14980: F, t15797: F, t1761: F, t3487: F, t3593: F, t4945: F, t498: F, t5055: F, t5089: F, t6244: F) -> (F, F, F, F, F, F, F) {
    let t19232 = t6151 * t225;
    let t19234 = t6153 * t225;
    let t19249 = t6239 * t225;
    let t19253 = t1720 * t5052;
    let t19256 = t4940 * t1751;
    let t19259 = t18571 * t491;
    let t19261 = -t1252 * t19232 - F::cast_from(2.0_f64) * t1252 * t19234 - t1252 * t19249 - F::cast_from(2.0_f64) * t14972 * t1761 - F::cast_from(2.0_f64) * t14980 * t1761 - F::cast_from(2.0_f64) * t15797 * t1761 + F::cast_from(2.0_f64) * t19253 * t498 + F::cast_from(2.0_f64) * t19256 * t498 + t19259 * t498 + F::cast_from(2.0_f64) * t3487 * t6244 + F::cast_from(2.0_f64) * t3593 * t6244 - F::cast_from(2.0_f64) * t4945 * t5089 - F::cast_from(2.0_f64) * t5055 * t5089;
    (t19232, t19234, t19249, t19253, t19256, t19259, t19261)
}
