//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 788/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk788<F: Float>(t70748: F, t70754: F, t73767: F, t73770: F, t73773: F, t73776: F, t73779: F, t73783: F, t15478: F, t16043: F, t3351: F, t3352: F, t44187: F, t515: F, t44239: F, t15457: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t76635 = 0.19863479950205658386e-4 * t70748;
    let t76637 = 0.99317399751028291929e-5 * t70754;
    let t76638 = 0.47885174879960069325e-4 * t73767;
    let t76639 = 0.19709219354514038085e-5 * t73770;
    let t76640 = 0.59127658063542114255e-5 * t73773;
    let t76641 = 0.59127658063542114255e-5 * t73776;
    let t76642 = 0.19709219354514038085e-5 * t73779;
    let t76643 = 0.19709219354514038085e-5 * t73783;
    let t76647 = t16043 * t15478;
    let t76648 = 0.12769379967989351819e-4 * t76647;
    let t76651 = t3351 * t3352 * t515 * t44187;
    let t76652 = 0.12769379967989351819e-4 * t76651;
    let t76655 = t3351 * t3352 * t515 * t44239;
    let t76656 = 0.12769379967989351819e-4 * t76655;
    let t76657 = t16043 * t15457;
    (t76635, t76637, t76638, t76639, t76640, t76641, t76642, t76643, t76648, t76652, t76656, t76657)
}
