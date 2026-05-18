//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1220/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1220<F: Float>(t5230: F, t68: F, t12240: F, t5335: F, t1352: F, t16040: F, t12189: F, t1804: F, t12188: F, t12190: F, t12194: F, t12196: F, t12197: F, t12200: F, t12205: F, t12209: F, t12212: F, t12228: F) -> (F, F, F, F) {
    let t16060 = t5230 * t68;
    let t16065 = t5335 * t12240;
    let t16068 = t16040 * t1352;
    let t16078 = t12189 * t1804;
    let t16080 = -t12188 - F::new(0.25925925925925925926e-1) * t12190 - t12194 + t12196 + F::new(0.38888888888888888888e-2) * t12197 - F::new(0.10555555555555555555e-1) * t12200 - F::new(0.25e-2) * t12205 + F::new(0.83333333333333333332e-3) * t12209 - F::new(0.11666666666666666666e-1) * t12212 + F::new(0.33333333333333333332e-2) * t12228 - F::new(0.12962962962962962962e-1) * t16078;
    (t16060, t16065, t16068, t16080)
}
