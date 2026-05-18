//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1024/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1024<F: Float>(t75725: F, t69976: F, t69983: F, t71582: F, t75689: F, t75692: F, t75695: F, t75700: F, t75703: F, t75718: F, t77782: F, t77785: F, t77788: F, t77791: F, t77792: F, t77793: F, t77794: F) -> F {
    let t77795 = F::new(0.44903406381989282115e-1) * t75725;
    let t77796 = F::new(0.54549323308490683461e-1) * t69976;
    let t77797 = F::new(0.72732431077987577948e-1) * t69983;
    let t77798 = -F::new(0.81756761766873046877e-6) * t75689 + F::new(0.52557918278704101564e-6) * t75692 + F::new(0.87596530464506835935e-6) * t75695 - F::new(0.87596530464506835935e-6) * t75700 + F::new(0.17519306092901367188e-6) * t75703 - t77782 - t77785 + t77788 + t77791 - t75718 - t77792 + t77793 + t77794 - t77795 + t77796 - t77797 + t71582;
    t77798
}
