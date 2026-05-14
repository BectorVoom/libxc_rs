//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 790/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk790<F: Float>(t73743: F, t73752: F, t73755: F, t73758: F, t73761: F, t70748: F, t70754: F, t73767: F, t73770: F, t73773: F, t73776: F, t73779: F, t73783: F, t73746: F, t73749: F, t73764: F, t73787: F) -> (F,) {
    let t76628 = 0.19709219354514038085e-5 * t73743;
    let t76631 = 0.3830813990396805546e-4 * t73752;
    let t76632 = 0.7661627980793611092e-4 * t73755;
    let t76633 = 0.15323255961587222184e-3 * t73758;
    let t76634 = 0.15961724959986689775e-4 * t73761;
    let t76635 = 0.19863479950205658386e-4 * t70748;
    let t76637 = 0.99317399751028291929e-5 * t70754;
    let t76638 = 0.47885174879960069325e-4 * t73767;
    let t76639 = 0.19709219354514038085e-5 * t73770;
    let t76640 = 0.59127658063542114255e-5 * t73773;
    let t76641 = 0.59127658063542114255e-5 * t73776;
    let t76642 = 0.19709219354514038085e-5 * t73779;
    let t76643 = 0.19709219354514038085e-5 * t73783;
    let t76645 = -t76628 - 0.4379826523225341797e-6 * t73746 - 0.35038612185802734376e-6 * t73749 - t76631 + t76632 - t76633 - t76634 - t76635 - 0.52557918278704101564e-6 * t73764 + t76637 - t76638 - t76639 + t76640 - t76641 - t76642 + t76643 - 0.87596530464506835935e-6 * t73787;
    (t76645,)
}
