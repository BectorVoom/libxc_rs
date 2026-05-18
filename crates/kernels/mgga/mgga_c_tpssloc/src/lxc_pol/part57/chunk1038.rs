//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1038/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1038<F: Float>(t22685: F, t31618: F, t6330: F, t6637: F, t122448: F, t1825: F, t22633: F, t6976: F, t115435: F, t122475: F, t122503: F, t127382: F, t127386: F, t127391: F, t1336: F, t31636: F, t33289: F, t5234: F, t6378: F, t6420: F, t8634: F) -> F {
    let t128860 = t22685 * t6637 * t31618 * t6330;
    let t128865 = t22633 * t6976 * t122448 * t1825;
    let t128874 = t115435 + t6378 * t8634 + F::new(0.49348022005446793095e-1) * t128860 + t127382 - F::new(0.38381794893125283518e-1) * t122503 + F::new(0.3289868133696452873e-1) * t128865 - t127386 - t127391 - t1336 * t31636 * t6420 - F::new(2.0) * t5234 * t33289 - F::new(2.0) * t1336 * t122475 * t1825;
    t128874
}
