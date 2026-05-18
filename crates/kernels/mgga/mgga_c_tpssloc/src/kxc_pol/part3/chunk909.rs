//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 909/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk909<F: Float>(t10273: F, t973: F, t2769: F, t40: F, t698: F, t986: F, t135: F, t3010: F, t241: F, t625: F, t281: F, t283: F) -> (F, F, F, F, F, F) {
    let t10274 = t973 * t10273;
    let t10276 = t2769 * t40;
    let t10277 = F::new(1.0) / t10276;
    let t10286 = t698 * t986;
    let t10287 = t973 * t10286;
    let t10289 = t135 * t3010;
    let t10290 = t973 * t10289;
    let t10292 = t625 * t241;
    let t10294 = t281 * t10292 * t283;
    (t10274, t10277, t10287, t10290, t10292, t10294)
}
