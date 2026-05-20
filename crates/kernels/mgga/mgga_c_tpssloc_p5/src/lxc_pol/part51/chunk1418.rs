//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1418/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1418<F: Float>(t119737: F, t119743: F, t119766: F, t121275: F, t121779: F, t121861: F, t1649: F, t1877: F, t24191: F, t24339: F, t2522: F, t25927: F, t25938: F, t26756: F, t31430: F, t33065: F, t33476: F, t33531: F, t7114: F, t83555: F, t84797: F, t8566: F, t8586: F, t92276: F) -> F {
    let t122072 = -t1877 * t7114 * t119766 / F::new(2.0) - t1877 * t7114 * t119737 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t84797 * t33531 + t26756 * t119743 - t1877 * t24339 * t33065 / F::new(2.0) + t1877 * t31430 * t1649 / F::new(2.0) + t26756 * t25927 * t121779 - t1877 * t92276 * t8586 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t8566 * t25938 - F::new(3.0) / F::new(2.0) * t24191 * t83555 * t33476 + F::new(3.0) * t24191 * t25927 * t121275 - t121861;
    t122072
}
