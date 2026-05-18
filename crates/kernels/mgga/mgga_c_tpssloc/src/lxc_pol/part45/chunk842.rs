//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 842/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk842<F: Float>(t10109: F, t1911: F, t10143: F, t25: F, t28: F, t1868: F, t671: F) -> (F, F, F, F) {
    let t25169 = t10109 * t1911;
    let t25373 = t10143 * t25;
    let t25927 = t10143 * t28;
    let t26103 = t1868 * t671;
    (t25169, t25373, t25927, t26103)
}
