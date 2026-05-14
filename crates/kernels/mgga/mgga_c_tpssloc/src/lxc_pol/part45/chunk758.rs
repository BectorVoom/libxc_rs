//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 758/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk758<F: Float>(t1877: F, t2057: F, t2249: F, t22951: F, t22961: F, t22964: F, t22968: F, t23296: F, t23299: F, t23302: F, t24191: F, t24335: F, t24339: F, t24344: F, t25: F, t2522: F, t4314: F, t606: F, t6542: F, t6671: F, t7110: F, t7114: F) -> (F,) {
    let t24355 = 3.0 * t4314 * t2057 * t22951 + 3.0 * t2522 * t7110 * t6542 - 3.0 * t24191 * t22961 + 3.0 * t2522 * t2057 * t22964 + 3.0 / 2.0 * t2522 * t2057 * t22968 + t1877 * t24335 * t25 / 2.0 - t1877 * t24339 * t6671 + t1877 * t7110 * t606 + t1877 * t24344 * t23296 - t1877 * t7114 * t23299 - t1877 * t7114 * t23302 / 2.0 + t1877 * t2057 * t2249 / 2.0;
    (t24355,)
}
