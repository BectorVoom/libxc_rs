//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2333/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2333<F: Float>(t27700: F, t95588: F, t18975: F, t7345: F, t18332: F, t7310: F, t1222: F, t29606: F, t1748: F, t18584: F, t24741: F, t27580: F, t27604: F, t27655: F, t27687: F, t27714: F, t5030: F, t6232: F, t7999: F, t8031: F, t8035: F, t86167: F, t95452: F, t95662: F, t95702: F) -> F {
    let t104425 = t95588 * t27700;
    let t104435 = t7345 * t18975;
    let t104441 = t7310 * t18332;
    let t104445 = t29606 * t1222;
    let t104449 = -F::cast_from(0.16149102437656156342e-2_f64) * t104425 - t95662 - F::cast_from(0.16149102437656156342e-2_f64) * t27580 * t8035 + F::cast_from(0.20186378047070195428e-3_f64) * t27714 * t8035 + F::cast_from(0.20186378047070195428e-3_f64) * t8031 * t27655 - t86167 * t6232 / F::new(1536.0) + F::new(5.0) / F::new(10368.0) * t104435 + t95452 * t1748 / F::new(216.0) + t27604 * t5030 / F::new(216.0) + t104441 / F::new(648.0) + t7999 * t27687 / F::new(27.0) + t104445 / F::new(2304.0) - t24741 * t18584 / F::new(1152.0) + t95702;
    t104449
}
