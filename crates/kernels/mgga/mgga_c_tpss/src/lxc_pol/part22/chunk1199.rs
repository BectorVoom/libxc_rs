//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1199/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1199<F: Float>(t10502: F, t10514: F, t10667: F, t20011: F, t1497: F, t2433: F, t31814: F, t33: F, t64248: F, t20047: F, t63844: F, t1006: F, t3610: F, t2133: F, t3683: F, t823: F) -> (F, F, F, F, F, F, F, F) {
    let t64770 = t10502 * t10514;
    let t64870 = t20011 * t10667;
    let t64876 = t1497 * t2433;
    let t64879 = t31814 * t33;
    let t64880 = t64879 * t64248;
    let t64888 = t20047 * t63844;
    let t64896 = t1006 * t3610;
    let t64905 = t1497 * t2133;
    let t64914 = t823 * t1006 * t3683;
    (t64770, t64870, t64876, t64880, t64888, t64896, t64905, t64914)
}
