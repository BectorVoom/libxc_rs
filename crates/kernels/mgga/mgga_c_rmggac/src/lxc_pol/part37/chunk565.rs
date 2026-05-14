//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 565/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk565<F: Float>(t15908: F, t15303: F, t15307: F, t15331: F, t15337: F, t15342: F, t15345: F, t15348: F, t15351: F, t15354: F, t15357: F, t15364: F, t15368: F, t15377: F, t15380: F, t15389: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15909 = 0.19957069503106347607e-1 * t15908;
    let t15910 = 0.93188427318671584242e-2 * t15303;
    let t15911 = 0.15531404553111930707e-1 * t15307;
    let t15914 = 0.58171619854173713844e-5 * t15331;
    let t15915 = 0.87596530464506835932e-6 * t15337;
    let t15916 = 0.87596530464506835932e-6 * t15342;
    let t15917 = 0.17519306092901367187e-6 * t15345;
    let t15918 = 0.43798265232253417968e-6 * t15348;
    let t15919 = 0.35038612185802734374e-6 * t15351;
    let t15920 = 0.52557918278704101561e-6 * t15354;
    let t15921 = 0.52557918278704101561e-6 * t15357;
    let t15922 = 0.58171619854173713844e-5 * t15364;
    let t15923 = 0.17451485956252114153e-4 * t15368;
    let t15924 = 0.58171619854173713844e-5 * t15377;
    let t15925 = 0.58171619854173713844e-5 * t15380;
    let t15927 = 0.35038612185802734374e-6 * t15389;
    (t15909, t15910, t15911, t15914, t15915, t15916, t15917, t15918, t15919, t15920, t15921, t15922, t15923, t15924, t15925, t15927)
}
