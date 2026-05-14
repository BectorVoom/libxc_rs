//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1139/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1139<F: Float>(t2749: F, t606: F, t23285: F, t2752: F, t2745: F, t10143: F, t6665: F, t2379: F, t13487: F, t10046: F, t1880: F, t214: F, t225: F, t258: F, t1888: F, t23270: F, t2717: F, t2742: F, t865: F) -> (F, F, F, F, F, F, F, F) {
    let t81521 = t606 * t2749;
    let t81525 = t23285 * t2752;
    let t81529 = t606 * t2745;
    let t81539 = t6665 * t10143;
    let t81543 = t606 * t2379;
    let t81547 = t2752 * t606;
    let t81548 = t81547 * t13487;
    let t81554 = t1880 * t214 * t10046 * t225 * t258;
    let t81559 = t1888 * t23270 * t2717 * t2742 * t865;
    (t81521, t81525, t81529, t81539, t81543, t81548, t81554, t81559)
}
