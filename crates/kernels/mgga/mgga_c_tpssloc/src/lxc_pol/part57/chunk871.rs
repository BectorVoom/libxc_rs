//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 871/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk871<F: Float>(t114732: F, t114734: F, t114737: F, t114739: F, t123576: F, t123578: F, t126325: F, t126328: F, t126332: F, t126334: F, t126337: F, t126339: F, t126341: F, t127908: F, t101355: F, t101509: F, t112863: F, t114865: F, t114892: F, t121629: F, t126372: F, t126385: F, t126398: F, t126399: F, t126404: F, t127889: F, t127896: F, t1912: F, t218: F, t25348: F, t259: F, t28317: F, t28432: F, t7087: F, t7842: F) -> (F, F) {
    let t127916 = t123576 - 0.16149102437656156341e-2 * t126325 + 0.32298204875312312682e-2 * t126328 - t123578 + t114732 - t114734 + 0.67826230238155856632e-1 * t126332 + t114737 + t114739 + 5.0 / 192.0 * t126334 + 0.19378922925187387609e-1 * t126337 - t126339 / 96.0 - t126341 / 192.0;
    let t127917 = t127908 + t127916;
    let t127926 = t126372 + t112863 - t101355 * t1912 + 0.3289868133696452873e-1 * t127889 - 2.0 * t25348 * t7842 - t126385 + 0.82246703342411321825e-2 * t127896 - t114865 + t114892 + t218 * t127917 * t259 - 0.38381794893125283518e-1 * t121629 - t126398 + 2.0 * t7087 * t28317 + t126399 + t126404 - t7087 * t28432 - 2.0 * t101509 * t1912;
    (t127917, t127926)
}
