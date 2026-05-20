//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1947/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1947<F: Float>(t1458: F, t6862: F, t4028: F, t6535: F, t19577: F, t8643: F, t22574: F, t7458: F, t2314: F, t7461: F, t4034: F, t1873: F, t5107: F) -> (F, F, F, F, F, F, F, F) {
    let t25965 = t6862 * t1458;
    let t25969 = F::new(2.0) * t4028 * t6535;
    let t25971 = t8643 * t19577;
    let t25973 = F::new(3.0) * t22574 * t25971;
    let t25975 = F::new(2.0) * t7458 * t6535;
    let t25977 = F::new(2.0) * t2314 * t7461;
    let t25979 = F::new(2.0) * t4034 * t7461;
    let t25980 = t5107 * t1873;
    (t25965, t25969, t25971, t25973, t25975, t25977, t25979, t25980)
}
