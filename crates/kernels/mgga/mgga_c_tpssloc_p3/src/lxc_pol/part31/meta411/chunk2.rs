//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1508/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1508<F: Float>(t12188: F, t12190: F, t12194: F, t12196: F, t12200: F, t1315: F, t16101: F, t19768: F, t19771: F, t19776: F, t19779: F, t19783: F, t19787: F, t5195: F) -> F {
    let t19790 = -t12188 - F::cast_from(0.12962962962962962963e-1_f64) * t12190 - F::cast_from(0.24999999999999999999e-2_f64) * t19768 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t19771 + F::cast_from(0.8333333333333333333e-3_f64) * t19776 - t12194 + t12196 - F::cast_from(0.52777777777777777776e-2_f64) * t12200 - F::cast_from(0.11666666666666666666e-1_f64) * t19779 - F::cast_from(0.19999999999999999999e-1_f64) * t16101 * t19783 + F::cast_from(0.99999999999999999996e-2_f64) * t5195 * t19787;
    t19790
}
