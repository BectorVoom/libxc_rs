//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1348/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1348<F: Float>(t100165: F, t100254: F, t100324: F, t100449: F, t1539: F, t1949: F, t21138: F, t21458: F, t23601: F, t23678: F, t23696: F, t25516: F, t28602: F, t28670: F, t4669: F, t5677: F, t6687: F, t6784: F, t6785: F, t83245: F, t89310: F, t89366: F, t89473: F) -> F {
    let t106176 = F::new(6.0) * t4669 * t28602 - F::new(0.82246703342411321826e-2) * t100254 - F::new(0.54831135561607547884e-2) * t89310 - F::new(0.82246703342411321825e-2) * t6687 * t21458 * t1949 + F::new(0.16449340668482264365e-1) * t83245 * t100165 * t23678 * t1539 + F::new(0.82246703342411321826e-2) * t6687 * t6784 * t100449 * t1539 + F::new(0.24674011002723396548e-1) * t23601 * t89473 * t28670 + F::new(0.10966227112321509577e-1) * t6687 * t23696 * t25516 * t5677 - F::new(0.18277045187202515961e-2) * t89366 + F::new(0.82246703342411321826e-2) * t100324 + F::new(0.16449340668482264365e-1) * t6687 * t6784 * t6785 * t21138;
    t106176
}
