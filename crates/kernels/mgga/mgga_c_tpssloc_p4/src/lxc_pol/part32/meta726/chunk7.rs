//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2348/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2348<F: Float>(t5415: F, t55: F, t16558: F, t17635: F, t17686: F, t17691: F, t1860: F, t1864: F, t24498: F, t26090: F, t27311: F, t27332: F, t27356: F, t27364: F, t29474: F, t29475: F, t29478: F, t29481: F, t3961: F, t3966: F, t607: F, t6486: F, t6495: F, t6509: F, t67: F, t7246: F, t7251: F, t7428: F, t7432: F, t7445: F, t83803: F, t85539: F, t96025: F, t96157: F, t96393: F) -> F {
    let t104818 = t5415 * t55;
    let t104858 = -t7428 * t27311 / F::cast_from(3.0_f64) - t6486 * t29475 / F::cast_from(6.0_f64) - t1860 * (-F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t104818 * t607 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t96157 * t3961 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t27356 * t3966 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t85539 * t17686 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t24498 * t17691 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t24498 * t17635 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7251 * t16558 + t83803) * t67 * t1864 / F::cast_from(6.0_f64) - t1860 * t29474 * t6509 / F::cast_from(6.0_f64) - t6486 * t29478 / F::cast_from(3.0_f64) - t1860 * t27364 * t7445 / F::cast_from(3.0_f64) + t6495 * t29475 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t27332 * t26090 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t29478 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t96393 + t6495 * t29481 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t96025 * t7432;
    t104858
}
