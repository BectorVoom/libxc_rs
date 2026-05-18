//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 883/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk883<F: Float>(t1912: F, t2054: F, t25188: F, t25348: F, t26700: F, t30640: F, t31321: F, t32791: F, t32794: F, t32811: F, t32817: F, t33372: F, t33399: F, t33416: F, t33420: F, t33423: F, t33430: F, t33433: F, t33463: F, t4147: F, t4268: F, t7087: F, t7538: F, t855: F, t8553: F, t8563: F) -> F {
    let t33465 = -F::new(0.82246703342411321825e-2) * t33372 - t32791 - t32794 - t4268 * t8563 - t855 * t33399 - t31321 - t25348 * t2054 + t32811 - t30640 + t32817 + t33416 - t4147 * t8563 - F::new(0.16449340668482264365e-1) * t33420 - F::new(0.82246703342411321825e-2) * t33423 - t7087 * t7538 - t25188 * t2054 + F::new(0.82246703342411321825e-2) * t33430 + F::new(2.0) * t855 * t33433 + F::new(2.0) * t4147 * t8553 + F::new(2.0) * t4268 * t8553 - t26700 * t1912 + t33463;
    t33465
}
