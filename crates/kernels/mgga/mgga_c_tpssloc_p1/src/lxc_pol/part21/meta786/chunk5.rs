//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2731/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2731<F: Float>(t1372: F, t6387: F, t6414: F, t12259: F, t1336: F, t1352: F, t1380: F, t16033: F, t16060: F, t16065: F, t16068: F, t16416: F, t1825: F, t19654: F, t19674: F, t19761: F, t19810: F, t3777: F, t5230: F, t5234: F, t5250: F, t5333: F, t5334: F, t5336: F, t5339: F, t5341: F, t5344: F, t55039: F, t57354: F, t6420: F) -> (F, F, F) {
    let t57607 = t1372 * t6387;
    let t57618 = t1372 * t6414;
    let t57631 = -t12259 * t1336 * t6420 - F::new(2.0) * t1336 * t1380 * t57354 - F::new(2.0) * t1336 * t1825 * t55039 - F::new(2.0) * t1352 * t5344 * t57607 + F::new(8.0) * t5230 * t5333 * t5336 + F::new(4.0) * t5250 * t5334 * t57618 - F::new(2.0) * t16033 * t19761 - F::new(4.0) * t16060 * t5339 - F::new(4.0) * t16060 * t5341 + F::new(4.0) * t16065 * t19654 - F::new(4.0) * t16068 * t19810 - F::new(4.0) * t16416 * t5234 - F::new(2.0) * t19674 * t3777;
    (t57607, t57618, t57631)
}
