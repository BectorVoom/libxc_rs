//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1791/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1791<F: Float>(t13602: F, t13598: F, t13613: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13645: F, t13647: F, t10300: F, t10556: F, t10558: F, t10560: F, t10562: F, t10675: F, t10676: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13551: F, t13552: F, t13557: F, t13561: F, t13563: F, t13592: F, t13616: F, t13624: F, t13626: F) -> F {
    let t13650 = F::cast_from(0.19931111111111111111e0_f64) * t13602;
    let t13652 = -F::new(0.1898925e1) * t13630 - F::new(0.9494625e0) * t13632 + F::cast_from(0.142419375e1_f64) * t13635 - F::new(0.76790625e-1) * t13638 + F::new(0.1898925e1) * t13640 - F::cast_from(0.91285185185185185185e-1_f64) * t13642 + t13645 - F::cast_from(0.82156666666666666667e-1_f64) * t13647 - F::cast_from(0.13287407407407407408e0_f64) * t13598 + t13650 - F::cast_from(0.29896666666666666667e0_f64) * t13613;
    let t13654 = -F::cast_from(0.54771111111111111112e-1_f64) * t13530 - F::cast_from(0.27385555555555555556e-1_f64) * t13534 - F::cast_from(0.36514074074074074075e-1_f64) * t13539 + F::cast_from(0.32862666666666666666e0_f64) * t13544 + F::cast_from(0.16431333333333333333e0_f64) * t13548 - t13551 + F::cast_from(0.36514074074074074074e-1_f64) * t13552 + F::cast_from(0.16431333333333333333e0_f64) * t13557 - F::cast_from(0.49293999999999999999e0_f64) * t13561 + F::cast_from(0.13287407407407407408e0_f64) * t13563 + t13592 - t10675 - t10676 + F::new(0.3071625e0) * t13616 - F::cast_from(0.10954222222222222222e0_f64) * t10300 - F::cast_from(0.26574814814814814816e0_f64) * t10556 + F::cast_from(0.66437037037037037038e-1_f64) * t10558 - F::cast_from(0.19931111111111111111e0_f64) * t10560 + F::cast_from(0.99655555555555555557e-1_f64) * t10562 + F::new(0.3071625e0) * t13624 + F::new(0.15358125e0) * t13626 + t13652;
    t13654
}
