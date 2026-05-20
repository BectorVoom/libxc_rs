//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 927/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk927<F: Float>(t1755: F, t5068: F, t1235: F, t1734: F, t1246: F, t491: F, t5011: F, t1215: F, t1932: F, t475: F) -> (F, F, F, F, F, F) {
    let t5069 = t1755 * t5068;
    let t5072 = t1235 * t1734;
    let t5073 = t5072 * t1246;
    let t5075 = t491 * t5011;
    let t5076 = t5075 * t1246;
    let t5079 = t1932 * t1215 * t475;
    (t5069, t5072, t5073, t5075, t5076, t5079)
}
