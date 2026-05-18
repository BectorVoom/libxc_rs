//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 985/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk985<F: Float>(t1824: F, t3792: F, t12345: F, t1831: F, t1362: F, t16060: F, t12339: F, t3866: F, t5314: F, t3865: F, t5234: F, t1369: F) -> (F, F, F, F, F, F, F) {
    let t16311 = t1824 * t3792;
    let t16317 = t12345 * t1831;
    let t16321 = t16060 * t1362;
    let t16325 = F::new(7.0) / F::new(576.0) * t12339 * t1831;
    let t16331 = F::new(7.0) / F::new(576.0) * t3866 * t5314;
    let t16336 = t5234 * t3865;
    let t16338 = F::new(7.0) / F::new(576.0) * t16336 * t1369;
    (t16311, t16317, t16321, t16325, t16331, t16336, t16338)
}
