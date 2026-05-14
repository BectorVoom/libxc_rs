//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1084/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1084<F: Float>(t213: F, t6330: F, t1307: F, t221: F, t5187: F, t5196: F, t12188: F, t12190: F, t12194: F, t12196: F, t12200: F, t1315: F, t16101: F, t19768: F, t19771: F, t19776: F, t19779: F, t5195: F) -> (F,) {
    let t19781 = t213 * t6330;
    let t19783 = t221 * t19781 * t1307;
    let t19787 = t221 * t5196 * t5187;
    let t19790 = -t12188 - 0.12962962962962962963e-1 * t12190 - 0.24999999999999999999e-2 * t19768 - 0.16666666666666666666e-2 * t1315 * t19771 + 0.8333333333333333333e-3 * t19776 - t12194 + t12196 - 0.52777777777777777776e-2 * t12200 - 0.11666666666666666666e-1 * t19779 - 0.19999999999999999999e-1 * t16101 * t19783 + 0.99999999999999999996e-2 * t5195 * t19787;
    (t19790,)
}
