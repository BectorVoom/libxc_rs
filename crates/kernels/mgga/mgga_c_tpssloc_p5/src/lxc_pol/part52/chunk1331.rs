//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1331/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1331<F: Float>(t3886: F, t7749: F, t1385: F, t1992: F, t22635: F, t1985: F, t8458: F, t90739: F, t114187: F, t114178: F, t114194: F, t120297: F, t120304: F, t120309: F, t120312: F, t120313: F, t120316: F, t1375: F, t16022: F, t1843: F, t26371: F, t26482: F, t31131: F, t3887: F, t5215: F, t6958: F, t6992: F, t8486: F) -> F {
    let t120317 = t3886 * t7749;
    let t120321 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t22635 * t120317 * t1385;
    let t120324 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t90739 * t8458;
    let t120327 = F::cast_from(0.82246703342411321825e-2_f64) * t114187;
    let t120328 = F::cast_from(4.0_f64) * t1375 * t3887 * t6992 * t7749 - t114194 * t1843 - t16022 * t8486 + F::cast_from(4.0_f64) * t26371 * t6958 + F::cast_from(4.0_f64) * t26482 * t6958 + F::cast_from(2.0_f64) * t31131 * t5215 - t114178 + t120297 + t120304 + t120309 - t120312 + t120313 - t120316 + t120321 - t120324 + t120327;
    t120328
}
