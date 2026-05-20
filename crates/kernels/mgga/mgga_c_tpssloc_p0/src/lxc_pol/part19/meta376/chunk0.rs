//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1401/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1401<F: Float>(t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t11778: F, t154: F) -> (F, F) {
    let t43808 = -F::new(16.0) / F::new(27.0) * t43748 - F::new(40.0) / F::new(81.0) * t43750 + F::new(8.0) / F::new(9.0) * t43780 + F::new(16.0) / F::new(9.0) * t43782 + F::new(16.0) / F::new(9.0) * t43784 - F::new(8.0) / F::new(3.0) * t43786 - F::new(4.0) / F::new(9.0) * t43788 + F::new(40.0) / F::new(9.0) * t43794 - F::new(8.0) * t43798 + F::new(8.0) * t43802 + t43806 / F::new(3.0);
    let t43809 = t154 * t11778;
    (t43808, t43809)
}
