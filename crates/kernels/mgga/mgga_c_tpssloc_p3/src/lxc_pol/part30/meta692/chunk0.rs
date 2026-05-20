//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2204/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2204<F: Float>(t5544: F, t606: F, t16662: F, t25: F, t2752: F, t28447: F, t28248: F, t776: F, t22960: F, t10143: F, t1408: F, t25374: F) -> (F, F, F, F, F, F) {
    let t98046 = t606 * t5544;
    let t98050 = t25 * t16662;
    let t98054 = t28447 * t2752;
    let t98058 = t28248 * t776;
    let t98059 = t22960 * t98058;
    let t98064 = t10143 * t1408;
    let t98065 = t98064 * t25374;
    (t98046, t98050, t98054, t98058, t98059, t98065)
}
