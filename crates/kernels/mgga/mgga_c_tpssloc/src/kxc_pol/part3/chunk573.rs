//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 573/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk573<F: Float>(t2742: F, t858: F, t259: F, t2592: F, t2594: F, t2597: F, t2711: F, t2713: F, t2720: F, t855: F, t866: F, t868: F) -> (F, F, F) {
    let t2743 = t858 * t2742;
    let t2745 = t259 * t2592 + F::new(2.0) * t259 * t2594 + t259 * t2711 - F::new(2.0) * t2597 * t866 - F::new(2.0) * t2713 * t866 + F::new(2.0) * t2720 * t855 - t2743 * t855;
    let t2749 = t868 * t868;
    (t2743, t2745, t2749)
}
