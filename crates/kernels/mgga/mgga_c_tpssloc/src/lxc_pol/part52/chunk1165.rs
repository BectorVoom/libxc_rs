//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1165/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1165<F: Float>(t1874: F, t26103: F, t6517: F, t6525: F, t532: F, t8492: F, t1307: F, t3701: F, t1983: F, t6876: F, t8490: F, t2015: F, t3886: F) -> (F, F, F, F, F, F, F) {
    let t31080 = t26103 * t1874;
    let t31082 = t6517 * t6525;
    let t31084 = t532 * t8492;
    let t31085 = t3701 * t1307;
    let t31086 = t31084 * t31085;
    let t31088 = F::new(3.0) * t1983 * t31086;
    let t31089 = t6876 * t8490;
    let t31090 = t3886 * t2015;
    (t31080, t31082, t31084, t31086, t31088, t31089, t31090)
}
