//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 799/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk799<F: Float>(t76912: F, t2227: F, t3351: F, t515: F, t618: F, t7231: F, t1528: F, t698: F, t14668: F, t17859: F, t14672: F, t74219: F, t14551: F, t8368: F, t74232: F, t70885: F) -> (F, F, F, F, F, F, F, F, F) {
    let t76913 = 0.53205749866622299248e-5 * t76912;
    let t76917 = t3351 * t7231 * t515 * t2227 * t618;
    let t76918 = 0.42564599893297839398e-5 * t76917;
    let t76922 = t3351 * t7231 * t515 * t698 * t1528;
    let t76923 = 0.42564599893297839398e-5 * t76922;
    let t76924 = t17859 * t14668;
    let t76925 = 0.42564599893297839398e-5 * t76924;
    let t76926 = t17859 * t14672;
    let t76927 = 0.12769379967989351819e-4 * t76926;
    let t76928 = 0.1921128438866447784e-2 * t74219;
    let t76930 = t8368 * t14551;
    let t76931 = 0.90915538847484472429e-2 * t76930;
    let t76932 = 0.68186654135613354325e-2 * t74232;
    let t76935 = 0.99317399751028291929e-5 * t70885;
    (t76913, t76918, t76923, t76925, t76927, t76928, t76931, t76932, t76935)
}
