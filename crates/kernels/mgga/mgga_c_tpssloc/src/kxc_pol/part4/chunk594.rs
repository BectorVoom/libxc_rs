//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 594/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk594<F: Float>(t1184: F, t3448: F, t1089: F, t460: F, t607: F, t3247: F, t461: F, t3293: F, t1191: F, t225: F, t1202: F, t1226: F) -> (F, F, F, F, F, F, F) {
    let t3449 = t3448 * t1184;
    let t3450 = t460 * t1089;
    let t3451 = t3450 * t607;
    let t3455 = t461 * t3247;
    let t3464 = F::new(5.0) / F::new(18.0) * t3293;
    let t3487 = t1191 * t225;
    let t3490 = t1202 * t1226;
    (t3449, t3450, t3451, t3455, t3464, t3487, t3490)
}
