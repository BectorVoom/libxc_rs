//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2628/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2628<F: Float>(t11697: F, t15709: F, t3577: F, t1226: F, t15764: F, t11832: F, t1706: F, t11665: F, t15608: F, t11838: F, t4889: F, t11841: F) -> (F, F, F, F, F, F) {
    let t53481 = t3577 * t11697 * t15709;
    let t53487 = t15764 * t1226;
    let t53490 = t1706 * t11832;
    let t53494 = t11665 * t15608;
    let t53496 = t4889 * t11838;
    let t53498 = t4889 * t11841;
    (t53481, t53487, t53490, t53494, t53496, t53498)
}
