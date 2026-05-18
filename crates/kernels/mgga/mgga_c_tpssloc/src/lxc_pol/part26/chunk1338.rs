//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1338/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1338<F: Float>(t2109: F, t83718: F, t22550: F, t7255: F, t83728: F, t83737: F, t22534: F, t22549: F, t24508: F, t24511: F, t24514: F, t24517: F, t6486: F, t7256: F, t7259: F, t83717: F, t83722: F, t83734: F, t83778: F) -> F {
    let t85463 = t2109 * t83718;
    let t85470 = t7255 * t22550;
    let t85473 = t2109 * t83728;
    let t85476 = t2109 * t83737;
    let t85479 = -t6486 * t24508 - t6486 * t24511 / F::new(2.0) + t22534 * t7256 + t22534 * t7259 - F::new(15.0) * t24514 * t83734 + F::new(30.0) * t83717 * t85463 - F::new(10.0) * t83722 * t24517 - F::new(5.0) * t83778 * t24517 - F::new(10.0) * t22549 * t85470 - F::new(10.0) * t22549 * t85473 - F::new(5.0) * t22549 * t85476;
    t85479
}
