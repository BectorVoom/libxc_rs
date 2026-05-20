//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1380/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1380<F: Float>(t10756: F, t10771: F, t14271: F, t1568: F, t1569: F, t17499: F, t17547: F, t21194: F, t21306: F, t2861: F, t2886: F, t5742: F, t5743: F, t5758: F, t5790: F, t69380: F, t76632: F, t76663: F, t76665: F, t76668: F, t76671: F, t77001: F, t77006: F, t77119: F, t77124: F, t77127: F, t77130: F) -> F {
    let t77390 = -t76663 - t76665 - t76668 + t76671 - t77001 - t77006 + F::new(36.0) * t2886 * t5743 * t5758 - F::new(8.0) * t2861 * t1569 * t21194 + F::cast_from(0.61524113149298439947e4_f64) * t10756 * t17499 * t5790 + F::cast_from(0.3859675079686208416e3_f64) * t14271 * t21306 + F::cast_from(0.12865583598954028054e3_f64) * t2886 * t69380 * t1568 - F::cast_from(0.11579025239058625248e4_f64) * t10771 * t17547 * t5742 - F::cast_from(0.19751673498613801407e-1_f64) * t76632 - t77119 + t77124 - t77127 + t77130;
    t77390
}
