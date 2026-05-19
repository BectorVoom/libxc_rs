//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1012/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1012<F: Float>(t27732: F, t462: F, t1170: F, t8077: F, t2121: F, t1201: F, t1244: F, t1729: F, t2152: F, t24856: F, t27572: F, t27574: F, t27722: F, t27725: F, t27728: F, t470: F, t4964: F, t7283: F, t7382: F, t7389: F, t7999: F, t8085: F) -> F {
    let t27733 = t462 * t27732;
    let t27736 = t1170 * t8077;
    let t27737 = t2121 * t27736;
    let t27739 = t1729 * t7389 + t4964 * t2152 + t1201 * t8085 - F::cast_from(0.73108180748810063843e-2_f64) * t27572 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27574 + t470 * t27722 + t1244 * t27725 - F::cast_from(0.91385225936012579807e-3_f64) * t24856 - F::cast_from(0.27415567780803773942e-2_f64) * t27728 - F::cast_from(0.21932454224643019153e-1_f64) * t7999 * t7382 + F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t27733 + F::cast_from(0.27415567780803773942e-2_f64) * t27737;
    t27739
}
