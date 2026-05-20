//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1812/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1812<F: Float>(t1041: F, t13950: F, t10370: F, t10372: F, t10377: F, t10381: F, t10385: F, t10390: F, t13750: F, t13751: F, t13758: F, t13762: F, t13767: F, t13942: F, t13946: F, t13948: F, t3070: F, t378: F, t4579: F) -> F {
    let t13952 = t1041 * t13950 / F::new(3456.0);
    let t13953 = -t13750 + F::new(19.0) / F::new(1728.0) * t13751 * t378 + t10370 / F::new(4608.0) + t10372 / F::new(1296.0) + t10377 + t10381 / F::new(81.0) + t10385 + t13758 + t10390 * t4579 / F::new(2304.0) + t3070 * t13762 / F::new(2304.0) + t13767 + t13942 * t378 / F::new(3072.0) - t13946 + t13948 + t13952;
    t13953
}
