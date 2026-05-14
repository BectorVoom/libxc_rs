//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 799/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk799<F: Float>(t225: F, t24237: F, t24235: F, t111: F, t7415: F, t112: F, t24954: F, t24542: F, t25: F, t40772: F, t10143: F, t606: F, t254: F, t853: F, t776: F, t865: F) -> (F, F, F, F, F, F, F, F, F) {
    let t85146 = t24237 * t225;
    let t85152 = t24235 * t225;
    let t85416 = t7415 * t111;
    let t85423 = t24954 * t112;
    let t85428 = t24542 * t111;
    let t86716 = t40772 * t25;
    let t86770 = t10143 * t606;
    let t87013 = t853 * t254;
    let t87036 = t776 * t865;
    (t85146, t85152, t85416, t85423, t85428, t86716, t86770, t87013, t87036)
}
