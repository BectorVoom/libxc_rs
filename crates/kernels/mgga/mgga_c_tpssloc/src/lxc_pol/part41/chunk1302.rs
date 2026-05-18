//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1302/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1302<F: Float>(t111916: F, t111961: F, t112006: F, t112049: F, t111845: F, t12524: F, t1458: F, t16521: F, t16524: F, t20173: F, t20176: F, t20181: F, t2199: F, t30109: F, t30112: F, t30315: F, t30385: F, t30390: F, t30534: F, t30608: F, t30611: F, t33185: F, t3938: F, t3941: F, t4072: F, t5493: F, t55353: F, t55388: F, t577: F, t66958: F, t671: F, t8212: F, t8273: F, t8294: F) -> (F, F) {
    let t112051 = t111916 + t111961 + t112006 + t112049;
    let t112062 = F::new(54.0) * t30112 * t20176 + F::new(0.135e2) * t111845 * t671 + F::new(0.135e2) * t66958 * t2199 + F::new(0.135e2) * t30109 * t5493 + F::new(54.0) * t55353 * t8294 + F::new(54.0) * t20173 * t30608 + F::new(54.0) * t3941 * t30315 * t1458 + F::new(54.0) * t3941 * t8273 * t4072 + F::new(27.0) * t16521 * t8273 + F::new(0.135e2) * t3938 * t30534 + F::new(54.0) * t12524 * t30608 + F::new(27.0) * t55388 * t8212 + F::new(27.0) * t3941 * t30534 * t671 + F::new(0.45e1) * t112051 * t577 + F::new(27.0) * t12524 * t30611 + F::new(27.0) * t30112 * t20181 + F::new(54.0) * t16524 * t30390 + F::new(54.0) * t33185 * t30385;
    (t112051, t112062)
}
