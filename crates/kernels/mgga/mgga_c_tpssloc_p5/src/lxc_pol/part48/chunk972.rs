//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 972/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk972<F: Float>(t113069: F, t113123: F, t114970: F, t114977: F, t114988: F, t114992: F, t115000: F, t115012: F, t115027: F, t13487: F, t1877: F, t1914: F, t193: F, t202: F, t23285: F, t23295: F, t2379: F, t24191: F, t24339: F, t24344: F, t2522: F, t2553: F, t2745: F, t2749: F, t31430: F, t31434: F, t31441: F, t31448: F, t4314: F, t6665: F, t7114: F, t776: F, t84766: F, t84791: F, t84800: F, t8566: F, t868: F, t870: F) -> F {
    let t115099 = -F::new(6.0) * t4314 * t7114 * t114977 + F::new(4.0) * t1877 * t24344 * t113123 + F::new(4.0) * t1877 * t84800 * t31448 - t1877 * t84791 * t1914 - F::new(2.0) * t1877 * t24339 * t6665 - t1877 * t7114 * t23285 + F::new(2.0) * t1877 * t24344 * t114988 + F::new(6.0) * t2522 * t31430 * t776 + F::new(12.0) * t24191 * t23295 * t13487 + F::new(2.0) * t1877 * t115027 * t2749 + t193 * t202 * t114970 * t870 + F::new(6.0) * t4314 * t8566 * t2379 - F::new(2.0) * t1877 * t114992 * t868 - F::new(6.0) * t1877 * t84766 * t115012 + F::new(3.0) * t2522 * t8566 * t2553 - F::new(6.0) * t2522 * t7114 * t113069 - F::new(3.0) * t2522 * t7114 * t115000 - t1877 * t31434 * t2745 - F::new(6.0) * t2522 * t24339 * t31441 - F::new(6.0) * t2522 * t31434 * t13487;
    t115099
}
