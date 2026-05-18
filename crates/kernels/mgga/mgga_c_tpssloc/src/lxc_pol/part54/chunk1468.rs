//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1468/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1468<F: Float>(t121142: F, t121144: F, t121159: F, t121160: F, t121162: F, t121165: F, t121169: F, t121174: F, t121177: F, t122917: F, t124728: F, t2040: F, t27863: F, t32679: F, t34170: F, t4034: F, t672: F, t7057: F) -> F {
    let t124933 = -F::new(2.0) * t122917 * t2040 - F::new(2.0) * t124728 * t672 - F::new(2.0) * t27863 * t7057 - F::new(2.0) * t34170 * t4034 + t121142 - t121144 - t121159 + t121160 - t121162 - t121165 - t121169 - t121174 + t121177 - t32679;
    t124933
}
