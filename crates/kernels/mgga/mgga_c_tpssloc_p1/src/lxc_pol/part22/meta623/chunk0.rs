//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2157/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2157<F: Float>(t53452: F, t11727: F, t52835: F, t11832: F, t1706: F, t15734: F, t3490: F, t11789: F, t1227: F, t248: F, t4733: F, t11712: F, t11913: F, t491: F) -> (F, F, F, F, F, F) {
    let t53453 = t53452 / F::new(2304.0);
    let t53472 = t52835 * t11727;
    let t53490 = t1706 * t11832;
    let t53515 = t3490 * t15734;
    let t53516 = t53515 / F::new(6912.0);
    let t53519 = t1227 * t248 * t11789 * t4733;
    let t53520 = t53519 / F::new(6912.0);
    let t53545 = t11712 * t11913 * t491;
    (t53453, t53472, t53490, t53516, t53520, t53545)
}
