//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1118/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1118<F: Float>(t1985: F, t8458: F, t90739: F, t114187: F, t114178: F, t114194: F, t120297: F, t120304: F, t120309: F, t120312: F, t120313: F, t120316: F, t120321: F, t1375: F, t16022: F, t1843: F, t26371: F, t26482: F, t31131: F, t3887: F, t5215: F, t6958: F, t6992: F, t7749: F, t8486: F) -> (F,) {
    let t120324 = 0.16449340668482264365e-1 * t1985 * t90739 * t8458;
    let t120327 = 0.82246703342411321825e-2 * t114187;
    let t120328 = 4.0 * t1375 * t3887 * t6992 * t7749 - t114194 * t1843 - t16022 * t8486 + 4.0 * t26371 * t6958 + 4.0 * t26482 * t6958 + 2.0 * t31131 * t5215 - t114178 + t120297 + t120304 + t120309 - t120312 + t120313 - t120316 + t120321 - t120324 + t120327;
    (t120328,)
}
