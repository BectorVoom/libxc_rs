//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 270/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk270<F: Float>(t829: F, t860: F, t235: F, t852: F, t226: F, t255: F, t808: F, t812: F, t858: F, t259: F, t799: F, t853: F, t855: F) -> (F, F, F, F, F) {
    let t861 = t860 * t829;
    let t863 = t235 * t852;
    let t865 = t226 * t863 + t255 * t808 - t812 * t861;
    let t866 = t858 * t865;
    let t868 = t259 * t799 + t259 * t853 - t855 * t866;
    (t861, t863, t865, t866, t868)
}
