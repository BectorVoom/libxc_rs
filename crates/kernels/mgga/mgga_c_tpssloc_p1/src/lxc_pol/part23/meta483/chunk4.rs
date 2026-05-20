//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1467/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1467<F: Float>(t11668: F, t11692: F, t1735: F, t1737: F, t1748: F, t21762: F, t21769: F, t3577: F, t3578: F, t467: F, t5971: F, t5979: F, t6219: F, t6230: F, t65935: F, t72304: F, t72307: F, t72597: F, t72600: F, t72632: F, t72634: F, t72648: F, t78506: F) -> F {
    let t79120 = F::new(5.0) / F::new(2304.0) * t3577 * t11668 * t6219 * t5971 - t72597 / F::new(216.0) - t72600 / F::new(36.0) - t3577 * t3578 * t1735 * t21769 / F::new(192.0) + t11692 * t3578 * t6230 * t5979 / F::new(768.0) - F::new(5.0) / F::new(2304.0) * t11692 * t11668 * t6230 * t5971 + F::new(5.0) / F::new(576.0) * t3577 * t11668 * t1735 * t21762 + F::new(1309.0) / F::new(486.0) * t78506 * t467 - t72632 / F::new(36.0) - t72304 * t1737 / F::new(48.0) - F::new(5.0) / F::new(324.0) * t72634 - F::new(5.0) / F::new(10368.0) * t65935 + t72307 * t1748 / F::new(72.0) - t72648 / F::new(36.0);
    t79120
}
