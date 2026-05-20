//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1844/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1844<F: Float>(t3014: F, t343: F, t6734: F, t1004: F, t6758: F, t1036: F, t6750: F, t1940: F, t3087: F, t354: F, t6759: F, t3: F, t6740: F) -> (F, F, F, F, F, F, F, F) {
    let t23547 = t3014 * t343;
    let t23548 = t23547 * t6734;
    let t23551 = t1004 * t6758;
    let t23554 = t6750 * t1036;
    let t23556 = t1940 * t3087;
    let t23557 = t354 * t23556;
    let t23560 = t6759 * t1036;
    let t23562 = t6740 * t3;
    (t23547, t23548, t23551, t23554, t23556, t23557, t23560, t23562)
}
