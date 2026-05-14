//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1010/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1010<F: Float>(t1799: F, t3701: F, t31084: F, t1983: F, t113: F, t1459: F, t1774: F, t31224: F, t33080: F, t33084: F, t33086: F, t33088: F, t33092: F, t33096: F, t33098: F, t33100: F, t33101: F, t33124: F, t33127: F, t33131: F, t33134: F, t33139: F, t33155: F, t33158: F, t510: F, t574: F, t8313: F) -> (F, F) {
    let t33159 = t3701 * t1799;
    let t33160 = t31084 * t33159;
    let t33162 = 3.0 * t1983 * t33160;
    let t33163 = -t113 * t33080 - 2.0 * t1459 * t31224 - t1774 * t8313 - t33124 * t510 + t33155 * t574 + t33084 - 4.0 * t33086 - 4.0 * t33088 - 2.0 * t33092 - t33096 - t33098 - t33100 - 4.0 * t33101 + 2.0 * t33127 + t33131 + 2.0 * t33134 - t33139 - t33158 - t33162;
    (t33160, t33163)
}
