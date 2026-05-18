//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1402/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1402<F: Float>(t115027: F, t118454: F, t118466: F, t121279: F, t121782: F, t1484: F, t16596: F, t1877: F, t23295: F, t24191: F, t24339: F, t2522: F, t25365: F, t25374: F, t26563: F, t26744: F, t31430: F, t31434: F, t31441: F, t33476: F, t4255: F, t4303: F, t6670: F, t7114: F, t7540: F, t868: F) -> F {
    let t121907 = F::new(2.0) * t115027 * t1877 * t25374 - F::new(3.0) * t118454 * t2522 * t7114 - F::new(3.0) * t118466 * t2522 * t7114 - F::new(3.0) * t121279 * t2522 * t7114 - t121782 * t1877 * t868 + F::new(3.0) * t1484 * t2522 * t31430 + F::new(6.0) * t16596 * t23295 * t24191 - F::new(3.0) * t16596 * t2522 * t31434 - t1877 * t24339 * t7540 - t1877 * t31434 * t4303 + F::new(6.0) * t23295 * t24191 * t25365 - F::new(3.0) * t24339 * t2522 * t33476 - F::new(3.0) * t2522 * t26744 * t31441 - F::new(6.0) * t26563 * t4255 * t6670;
    t121907
}
