//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1423/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1423<F: Float>(t835: F, t10913: F, t1860: F, t1864: F, t2244: F, t22490: F, t2250: F, t22502: F, t22505: F, t22512: F, t22513: F, t22516: F, t22534: F, t22551: F, t44: F, t607: F, t6486: F, t6490: F, t6492: F, t6495: F, t6500: F, t6506: F, t6509: F, t6510: F, t67: F, t83771: F, t83775: F, t83778: F, t83788: F, t83791: F, t83796: F, t9258: F, t9276: F, t9288: F) -> F {
    let t83803 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t835;
    let t83812 = F::cast_from(2.0_f64) * t6495 * t22516 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t6490 * t83771 + t6495 * t22490 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t83775 * t6492 - F::cast_from(5.0_f64) * t83778 * t22551 + t22534 * t6506 + t22534 * t6510 - t6486 * t22513 / F::cast_from(2.0_f64) - t6486 * t22516 - t1860 * (-F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t9276 * t44 + F::cast_from(220.0_f64) / F::cast_from(9.0_f64) * t83788 * t607 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t83791 * t2244 - F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t22502 * t2250 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t83796 * t9288 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t22505 * t10913 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6500 * t9258 + t83803) * t67 * t1864 / F::cast_from(6.0_f64) - t1860 * t22512 * t6509 / F::cast_from(2.0_f64);
    t83812
}
