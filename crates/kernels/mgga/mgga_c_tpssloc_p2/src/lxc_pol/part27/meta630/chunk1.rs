//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2120/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2120<F: Float>(t22986: F, t23270: F, t865: F, t86849: F, t4300: F, t776: F, t857: F, t1888: F, t2717: F, t25044: F, t2742: F, t23168: F, t25342: F) -> (F, F, F, F, F) {
    let t86852 = t22986 * t23270 * t86849 * t865;
    let t86857 = t22986 * t23270 * t857 * t4300 * t776;
    let t86862 = t1888 * t23270 * t2717 * t4300 * t865;
    let t86866 = t1888 * t23270 * t25044 * t2742;
    let t86868 = t23168 * t25342;
    (t86852, t86857, t86862, t86866, t86868)
}
