//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 714/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk714<F: Float>(t4854: F, t847: F, t2504: F, t4847: F, t854: F, t2515: F, t4826: F, t141: F, t4830: F, t861: F, t4834: F, t2499: F, t2512: F, t3746: F, t3795: F, t4828: F, t4832: F, t4836: F, t4848: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4855 = t847 * t4854;
    let t4861 = t2504 * t4847;
    let t4863 = t854 * t4854;
    let t4866 = t2515 * t4826;
    let t4867 = t141 * t4866;
    let t4869 = t861 * t4830;
    let t4870 = t141 * t4869;
    let t4872 = t861 * t4834;
    let t4873 = t141 * t4872;
    let t4875 = -0.9494625e0 * t4848 + 0.1898925e1 * t4855 + t2499 + 0.19931111111111111111e0 * t3746 - 0.19931111111111111111e0 * t4828 + 0.59793333333333333334e0 * t4832 - 0.29896666666666666667e0 * t4836 + 0.15358125e0 * t4861 + 0.3071625e0 * t4863 + t2512 + 0.10954222222222222222e0 * t3795 - 0.27385555555555555556e-1 * t4867 + 0.16431333333333333333e0 * t4870 - 0.82156666666666666667e-1 * t4873;
    (t4855, t4861, t4863, t4866, t4867, t4869, t4870, t4872, t4873, t4875)
}
