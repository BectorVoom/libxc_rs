//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2638/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2638<F: Float>(t11944: F, t1256: F, t14696: F, t15838: F, t1763: F, t193: F, t336: F, t3633: F, t43706: F, t4700: F, t51889: F, t51892: F, t51898: F, t51903: F, t51905: F, t51906: F, t51913: F, t51916: F, t51946: F, t53665: F, t53697: F, t53729: F) -> F {
    let t53735 = -t51889 + t51892 - F::new(6.0) * t4700 * t1763 * t43706 * t11944 - t51898 - F::new(3.0) * t4700 * t14696 * t3633 - t51903 - t51905 + F::new(6.0) * t4700 * t15838 * t51906 + t51913 - t51916 + t193 * t336 * (t51946 + t53665 + t53697 + t53729) * t1256;
    t53735
}
