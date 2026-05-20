//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1392/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1392<F: Float>(t2745: F, t870: F, t2553: F, t262: F, t2752: F, t1877: F, t2522: F, t4314: F, t776: F, t868: F, t9684: F, t9715: F, t9718: F, t9724: F, t9727: F, t9780: F, t9789: F, t9863: F, t9865: F, t9867: F, t9870: F) -> (F, F, F) {
    let t10126 = t2745 * t870;
    let t10130 = t262 * t2553;
    let t10134 = t2745 * t2752;
    let t10138 = F::new(9.0) * t10126 * t2522 * t776 + F::new(18.0) * t10130 * t4314 * t776 - F::new(3.0) * t10134 * t1877 * t868 + t9684 - t9715 - t9718 + t9724 + t9727 + t9780 - t9789 + t9863 + t9865 - t9867 + t9870;
    (t10126, t10134, t10138)
}
