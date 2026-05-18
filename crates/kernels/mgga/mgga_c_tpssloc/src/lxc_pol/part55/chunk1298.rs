//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1298/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1298<F: Float>(t125094: F, t125103: F, t125121: F, t125802: F, t125919: F, t125939: F, t125951: F, t125963: F, t1858: F, t8919: F, t2174: F, t8110: F) -> (F, F, F) {
    let t125966 = t125094 + t125103 + t125121 + t125802 + t125919 + t125939 + t125951 + t125963;
    let t125970 = t8919 * t1858;
    let t125975 = t8110 * t2174;
    (t125966, t125970, t125975)
}
