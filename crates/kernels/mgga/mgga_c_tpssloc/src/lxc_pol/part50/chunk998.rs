//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 998/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk998<F: Float>(t32934: F, t6799: F, t1948: F, t7593: F, t345: F, t1615: F, t8391: F, t1060: F, t30816: F, t7577: F, t30820: F, t7582: F, t1610: F, t8387: F, t1618: F, t1622: F, t1935: F, t30813: F, t30829: F, t30837: F, t30840: F, t378: F, t6742: F, t7574: F, t8384: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32935 = t6799 * t32934;
    let t32938 = t1948 * t7593;
    let t32939 = t345 * t32938;
    let t32943 = t8391 * t1615;
    let t32944 = t32943 * t1060;
    let t32948 = t7577 * t30816;
    let t32951 = t30820 * t7582;
    let t32954 = t1610 * t8387;
    let t32961 = t30813 + 0.40372756094140390856e-3 * t7574 * t8384 - 0.40372756094140390856e-3 * t1935 * t32948 + 0.40372756094140390856e-3 * t6742 * t32951 + t32954 * t378 / 1536.0 + t30829 * t1618 / 1536.0 + t30837 + t30840 * t1622 / 2304.0;
    (t32935, t32938, t32939, t32943, t32944, t32948, t32951, t32954, t32961)
}
