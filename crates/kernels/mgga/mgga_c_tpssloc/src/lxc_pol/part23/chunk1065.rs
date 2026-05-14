//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1065/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1065<F: Float>(t2374: F, t39516: F, t39325: F, t39497: F, t39500: F, t39506: F, t10108: F, t257: F, t68: F, t233: F, t9970: F, t252: F, t2632: F, t10021: F, t812: F, t841: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40793 = 0.1301229756036208781e0 * t2374 * t39516;
    let t40797 = 0.38025319932552508021e2 * t2374 * t39325;
    let t40799 = 0.67471172535210825684e-1 * t2374 * t39497;
    let t40801 = 0.86748650402413918736e-1 * t2374 * t39500;
    let t40803 = 0.38527786510141256862e1 * t2374 * t39506;
    let t40889 = 1.0 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40931 = 1.0 / t9970 / t233;
    let t40932 = t40931 * t252;
    let t40933 = t2632 * t2632;
    let t40965 = t812 * t841 * t10021;
    (t40793, t40797, t40799, t40801, t40803, t40890, t40931, t40932, t40933, t40965)
}
