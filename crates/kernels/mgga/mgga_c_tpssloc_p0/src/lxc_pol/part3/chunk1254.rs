//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1254/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1254<F: Float>(t12541: F, t12543: F, t1396: F, t1398: F, t1404: F, t16507: F, t16513: F, t16515: F, t16546: F, t1852: F, t1858: F, t3932: F, t3946: F, t5364: F, t5381: F, t580: F, t9203: F, t9205: F, t9207: F) -> F {
    let tv3rho31 = F::new(2.0) * t1396 * t5381 + t1398 * t16546 + F::new(2.0) * t1404 * t5364 + t16507 * t580 + t1852 * t3946 + t1858 * t3932 + t12541 + t12543 + t16513 + t16515 + t9203 + F::new(2.0) * t9205 + t9207;
    tv3rho31
}
