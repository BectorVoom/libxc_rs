//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1307/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1307<F: Float>(t1288: F, t2433: F, t19817: F, t44474: F, t2133: F, t10897: F, t30: F, t2116: F, t44350: F, t2436: F, t10514: F, t1398: F, t2428: F) -> (F, F, F, F, F, F, F, F) {
    let t63794 = t1288 * t2433;
    let t63797 = t19817 * t44474;
    let t63806 = t1288 * t2133;
    let t63817 = t30 * t10897;
    let t63823 = t1288 * t2116;
    let t63837 = t19817 * t44350;
    let t63840 = t2436 * t1288;
    let t63841 = t63840 * t10514;
    let t63844 = t1398 * t2428;
    (t63794, t63797, t63806, t63817, t63823, t63837, t63841, t63844)
}
