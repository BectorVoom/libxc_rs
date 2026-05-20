//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2203/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2203<F: Float>(t111: F, t12723: F, t2363: F, t649: F, t89: F, t9416: F, t11968: F, t12492: F, t12557: F, t1266: F, t12725: F, t12734: F, t12813: F, t12816: F, t12823: F, t12835: F, t1393: F, t1458: F, t1459: F, t1778: F, t1849: F, t19456: F, t2314: F, t2364: F, t4037: F, t652: F, t672: F, t9419: F) -> (F, F, F) {
    let t45632 = t12723 * t111;
    let t45637 = t649 * t2363;
    let t45640 = t89 * t9416;
    let t45648 = -F::new(2.0) * t11968 * t1458 * t652 - F::new(6.0) * t1266 * t12813 * t652 + t12492 * t1778 - F::new(6.0) * t12557 * t2314 - F::new(6.0) * t12725 * t2364 - F::new(12.0) * t12734 * t4037 + F::new(3.0) * t12816 * t1393 - F::new(6.0) * t12823 * t4037 - F::new(6.0) * t12835 * t2314 - F::new(6.0) * t1459 * t45637 - F::new(2.0) * t1459 * t45640 + t1849 * t9419 - F::new(6.0) * t19456 * t2364 - F::new(6.0) * t45632 * t672;
    (t45632, t45637, t45648)
}
