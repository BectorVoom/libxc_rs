//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1631/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1631<F: Float>(t475: F, t6739: F, t11882: F, t11616: F, t11621: F, t11625: F, t11640: F, t11869: F, t11872: F, t11877: F, t11881: F, t11884: F, t11888: F, t11890: F, t11893: F, t11897: F, t11904: F, t11907: F, t11910: F, t11914: F, t1201: F, t1244: F, t1247: F, t1249: F, t3565: F, t3604: F, t3610: F, t3613: F, t3617: F, t3621: F, t3624: F, t3626: F, t3628: F, t470: F, t494: F) -> (F, F, F) {
    let t11915 = t6739 * t475;
    let t11916 = t11882 * t11915;
    let t11918 = t11616 * t494 + F::new(3.0) * t3604 * t3621 + F::new(3.0) * t1244 * t11621 - F::new(3.0) * t3624 * t11625 + t1244 * t11640 + t470 * t11869 + F::new(6.0) * t3610 * t11872 + F::new(6.0) * t3604 * t3617 + F::new(3.0) * t11877 * t1247 + F::new(6.0) * t11881 * t11884 - F::new(6.0) * t11888 * t11890 + F::new(6.0) * t3610 * t11893 + F::new(3.0) * t1244 * t11897 + F::new(3.0) * t3565 * t1249 + F::new(3.0) * t1201 * t3628 + F::new(6.0) * t11904 * t3613 - F::new(3.0) * t11907 * t3626 - F::new(3.0) * t3624 * t11910 + t11914 * t11916;
    (t11915, t11916, t11918)
}
