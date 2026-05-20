//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1091/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1091<F: Float>(t25962: F, t25999: F, t26155: F, t26507: F, t3: F, t112: F, t7758: F, t16521: F, t1873: F, t16524: F, t7015: F, t5371: F, t6534: F) -> (F, F, F, F, F, F) {
    let t26509 = t25962 + t25999 + t26155 + t26507;
    let t26510 = t3 * t26509;
    let t26523 = t7758 * t112;
    let t26533 = F::new(0.135e2) * t16521 * t1873;
    let t26535 = F::new(27.0) * t16524 * t7015;
    let t26537 = F::new(0.135e2) * t5371 * t6534;
    (t26509, t26510, t26523, t26533, t26535, t26537)
}
