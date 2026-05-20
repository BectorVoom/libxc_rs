//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2734/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2734<F: Float>(t19731: F, t562: F, t12267: F, t1336: F, t1352: F, t1383: F, t16033: F, t16036: F, t16060: F, t16136: F, t16429: F, t19739: F, t19805: F, t20014: F, t3856: F, t3897: F, t5234: F, t5250: F, t5287: F, t5334: F, t5344: F, t5349: F, t564: F, t56914: F, t57465: F, t57545: F, t57618: F, t6454: F) -> (F, F) {
    let t57704 = t562 * t19731;
    let t57725 = F::new(4.0) * t1336 * t3897 * t56914 - F::new(4.0) * t1352 * t5344 * t57545 - F::new(2.0) * t1352 * t5344 * t57618 - F::new(4.0) * t16036 * t5287 * t5344 - F::new(2.0) * t19739 * t3856 * t5344 + F::new(4.0) * t5250 * t5334 * t57704 - t12267 * t6454 + F::new(2.0) * t1383 * t19805 - F::new(4.0) * t16033 * t20014 - F::new(4.0) * t16060 * t5349 - F::new(2.0) * t16136 * t5234 + F::new(4.0) * t16429 * t5234 + t564 * t57465;
    (t57704, t57725)
}
