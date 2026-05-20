//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 740/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk740<F: Float>(t1215: F, t3612: F, t1755: F, t1235: F, t1734: F, t1246: F, t491: F, t5011: F, t1932: F, t475: F, t1751: F, t493: F, t5052: F) -> (F, F, F, F, F, F, F, F) {
    let t5068 = t3612 * t1215;
    let t5069 = t1755 * t5068;
    let t5072 = t1235 * t1734;
    let t5073 = t5072 * t1246;
    let t5075 = t491 * t5011;
    let t5076 = t5075 * t1246;
    let t5079 = t1932 * t1215 * t475;
    let t5080 = t1755 * t5079;
    let t5083 = t1751 * t1215;
    let t5084 = t5083 * t1246;
    let t5086 = t493 * t5052;
    (t5068, t5069, t5073, t5076, t5079, t5080, t5084, t5086)
}
