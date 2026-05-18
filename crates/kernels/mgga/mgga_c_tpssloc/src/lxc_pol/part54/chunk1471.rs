//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1471/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1471<F: Float>(t121226: F, t121228: F, t121231: F, t121233: F, t121234: F, t121237: F, t12725: F, t24932: F, t26898: F, t27163: F, t27290: F, t27863: F, t27888: F, t33690: F, t7042: F, t7057: F, t7061: F, t7266: F, t7796: F, t8690: F, t8835: F) -> F {
    let t124969 = -F::new(2.0) * t12725 * t8835 - F::new(2.0) * t24932 * t7796 + F::new(3.0) * t26898 * t8690 - F::new(2.0) * t27163 * t7266 - F::new(2.0) * t27290 * t7042 - F::new(2.0) * t27863 * t7061 - F::new(2.0) * t27888 * t7796 - F::new(2.0) * t33690 * t7057 - t121226 - t121228 - t121231 - t121233 - t121234 - t121237;
    t124969
}
