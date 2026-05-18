//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1419/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1419<F: Float>(t1863: F, t83728: F, t1860: F, t1865: F, t22489: F, t22490: F, t22493: F, t22549: F, t22551: F, t6486: F, t6505: F, t6506: F, t6510: F, t83699: F, t83706: F, t83710: F, t83717: F, t83719: F, t83722: F, t83725: F) -> F {
    let t83729 = t1863 * t83728;
    let t83732 = t83699 * t1865 - t6486 * t22490 / F::new(2.0) - t1860 * t6505 * t22489 / F::new(2.0) - t1860 * t1863 * t83706 / F::new(6.0) - t83710 * t1865 / F::new(6.0) - t22493 * t6506 / F::new(2.0) - t22493 * t6510 / F::new(2.0) + F::new(30.0) * t83717 * t83719 - F::new(10.0) * t83722 * t22551 - F::new(10.0) * t22549 * t83725 - F::new(10.0) * t22549 * t83729;
    t83732
}
