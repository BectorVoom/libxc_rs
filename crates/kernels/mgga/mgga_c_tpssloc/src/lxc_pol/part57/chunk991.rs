//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 991/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk991<F: Float>(t127908: F, t127916: F, t101355: F, t101509: F, t112863: F, t114865: F, t114892: F, t121629: F, t126372: F, t126385: F, t126398: F, t126399: F, t126404: F, t127889: F, t127896: F, t1912: F, t218: F, t25348: F, t259: F, t28317: F, t28432: F, t7087: F, t7842: F) -> (F, F) {
    let t127917 = t127908 + t127916;
    let t127926 = t126372 + t112863 - t101355 * t1912 + F::new(0.3289868133696452873e-1) * t127889 - F::new(2.0) * t25348 * t7842 - t126385 + F::new(0.82246703342411321825e-2) * t127896 - t114865 + t114892 + t218 * t127917 * t259 - F::new(0.38381794893125283518e-1) * t121629 - t126398 + F::new(2.0) * t7087 * t28317 + t126399 + t126404 - t7087 * t28432 - F::new(2.0) * t101509 * t1912;
    (t127917, t127926)
}
