//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 295/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk295<F: Float>(t880: F, t906: F, t886: F, t897: F, t902: F, t910: F) -> (F, F, F) {
    let t926 = F::new(0.516475e0) * t880;
    let t929 = F::new(0.104195e0) * t906;
    let t931 = F::new(0.3529725e1) * t897 - t926 - F::new(0.516475e0) * t886 + F::new(0.6311625e0) * t902 - t929 - F::new(0.104195e0) * t910;
    (t926, t929, t931)
}
