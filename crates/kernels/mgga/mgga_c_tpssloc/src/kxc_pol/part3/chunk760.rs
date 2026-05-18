//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 760/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk760<F: Float>(t4202: F, t707: F, t1471: F, t706: F, t708: F, t1462: F, t2427: F, t2373: F, t2377: F, t2408: F, t4097: F, t4099: F, t4100: F, t4103: F, t4198: F, t4201: F) -> (F, F, F, F, F) {
    let t4204 = F::new(4.0) * t707 * t4202;
    let t4205 = t706 * t1471;
    let t4207 = F::new(4.0) * t4205 * t708;
    let t4209 = F::new(4.0) * t2427 * t1462;
    let t4210 = t4097 + t4099 + t4100 + t4103 + t4198 - t4201 + t2373 + t2377 + t4204 + t4207 + t4209 + t2408;
    (t4204, t4205, t4207, t4209, t4210)
}
