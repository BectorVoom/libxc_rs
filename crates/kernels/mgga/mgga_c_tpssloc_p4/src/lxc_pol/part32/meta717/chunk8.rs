//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2279/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2279<F: Float>(t89953: F, t97999: F, t10143: F, t1649: F, t25374: F, t5966: F, t776: F, t4303: F, t23788: F, t67164: F, t16944: F, t25891: F) -> (F, F, F, F, F, F) {
    let t100682 = t89953 * t97999;
    let t100688 = t10143 * t1649;
    let t100689 = t100688 * t25374;
    let t100692 = t5966 * t776;
    let t100696 = t1649 * t4303;
    let t100705 = t23788 * t67164;
    let t100708 = t25891 * t16944;
    (t100682, t100689, t100692, t100696, t100705, t100708)
}
